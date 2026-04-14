-- 1. Create the Auth user and database
CREATE USER zitadel WITH PASSWORD 'ZitadelSecurePassword123!';
CREATE DATABASE zitadel;
GRANT ALL PRIVILEGES ON DATABASE zitadel TO zitadel;

-- 2. Create the Rust App user and database
CREATE USER pathlight WITH PASSWORD 'PathlightSecurePassword123!';
CREATE DATABASE pathlight;
GRANT ALL PRIVILEGES ON DATABASE pathlight TO pathlight;