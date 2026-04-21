use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>,
}

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZitadelClaims {
    pub sub: String, // The User ID
    pub iss: String,
    pub aud: Vec<String>,
    pub exp: usize,
}

pub struct TokenValidator {
    jwks_url: String,
    issuer: String,
    audience: String,
    keys_cache: RwLock<HashMap<String, DecodingKey>>,
}

impl TokenValidator {
    pub fn new(issuer: String, audience: String) -> Self {
        // ZITADEL's standard JWKS endpoint
        let jwks_url = format!("{}/oauth/v2/keys", issuer);

        Self {
            jwks_url,
            issuer,
            audience,
            keys_cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn validate(&self, token: &str) -> Option<ZitadelClaims> {
        // 1. Read the header to find out which key ZITADEL used
        let header = match decode_header(token) {
            Ok(h) => h,
            Err(e) => {
                tracing::error!(
                    "Failed to decode JWT header (Is it an opaque token?): {}",
                    e
                );
                return None;
            }
        };
        let kid = match header.kid {
            Some(k) => k,
            None => {
                tracing::error!("JWT is missing the 'kid' field in header");
                return None;
            }
        };

        // 2. Check the cache for the key
        let mut has_key = {
            let cache = self.keys_cache.read().await;
            cache.contains_key(&kid)
        };

        // 3. If missing, trigger the async network fetch
        if !has_key {
            if let Ok(new_keys) = self.fetch_jwks().await {
                let mut cache = self.keys_cache.write().await;
                for jwk in new_keys.keys {
                    if let Ok(decoding_key) = DecodingKey::from_rsa_components(&jwk.n, &jwk.e) {
                        cache.insert(jwk.kid, decoding_key);
                    }
                }
            }
            // Check if the kid is now in the cache
            let cache = self.keys_cache.read().await;
            has_key = cache.contains_key(&kid);
        }

        // 4. Validate the token!
        if has_key {
            let cache = self.keys_cache.read().await;
            let key = cache.get(&kid).unwrap();

            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_issuer(&[self.issuer.clone()]);
            validation.set_audience(&[self.audience.clone()]);

            match decode::<ZitadelClaims>(token, key, &validation) {
                Ok(token_data) => return Some(token_data.claims),
                Err(e) => {
                    tracing::error!("JWT Signature or Claim Validation failed: {}", e);
                    return None;
                }
            }
        } else {
            tracing::error!("kid '{}' not found in JWKS cache after fetch", kid);
        }

        return None;
    }

    async fn fetch_jwks(&self) -> Result<JwksResponse, reqwest::Error> {
        reqwest::get(&self.jwks_url)
            .await?
            .json::<JwksResponse>()
            .await
    }
}
