import 'package:flutter/material.dart';
import 'package:oidc/oidc.dart';
import 'package:oidc_default_store/oidc_default_store.dart';

const String clientId = '368605239219126864';
const String issuerUrl = 'http://localhost:8080';
const String redirectUri = 'http://localhost:4444/auth.html';

Future<OidcUserManager> initAuth() async {
  var manager = OidcUserManager.lazy(
    discoveryDocumentUri: OidcUtils.getOpenIdConfigWellKnownUri(
      Uri.parse(issuerUrl),
    ),
    clientCredentials: const OidcClientAuthentication.none(clientId: clientId),
    store: OidcDefaultStore(),
    settings: OidcUserManagerSettings(
      redirectUri: Uri.parse(redirectUri),
      scope: ['openid', 'profile', 'email', 'offline_access'],
    ),
  );

  try {
    await manager.init();
  } catch (e) {
    debugPrint('OIDC Init Crash: $e');
  }
  return manager;
}

Future<void> performLogin(OidcUserManager manager) async {
  await manager.loginAuthorizationCodeFlow(
    options: const OidcPlatformSpecificOptions(
      web: OidcPlatformSpecificOptions_Web(
        navigationMode: OidcPlatformSpecificOptions_Web_NavigationMode.samePage,
      ),
    ),
    originalUri: Uri.parse('http://localhost:4444/'),
  );
}
