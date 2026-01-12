use std::{borrow::Cow, str, sync::LazyLock, time::Duration};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};

static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(JWT::default);

const DEFAULT_SECRET: &str = "1234567890";
const DEFAULT_AUDIENCE: &str = "aduience";
const DEFAULT_EXP: u64 = 60 * 60;
const DEFAULT_ISSUER: &str = "issuer";
#[derive(Debug, Clone)]
pub struct Principal {
    pub id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub exp: Duration,
    pub aduience: Cow<'static, str>,
    pub issuer: Cow<'static, str>,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            exp: Duration::from_secs(DEFAULT_EXP),
            aduience: Cow::Borrowed(DEFAULT_AUDIENCE),
            issuer: Cow::Borrowed(DEFAULT_ISSUER),
        }
    }
}

pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        let encode_secret = EncodingKey::from_secret(secret);
        let decode_secret = DecodingKey::from_secret(secret);
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&[&config.aduience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let expiration: Duration = config.exp;
        let audience = config.aduience.to_string();
        let issuer = config.issuer.to_string();
        Self {
            encode_secret,
            decode_secret,
            header,
            validation,
            expiration,
            audience,
            issuer,
        }
    }
    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!(
                "{}:{}:{}:{}",
                principal.id,
                principal.username,
                principal.roles.join(","),
                principal.permissions.join(",")
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        let token = jsonwebtoken::encode(&self.header, &claims, &self.encode_secret)?;
        Ok(token)
    }
    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let token_data =
            jsonwebtoken::decode::<Claims>(token, &self.decode_secret, &self.validation)?;
        let mut parts = token_data.claims.sub.splitn(4, ':');
        let id = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing id in token"))?
            .to_string();
        let username = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing username in token"))?
            .to_string();
        let roles_str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing roles in token"))?;
        let permissions_str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing permissions in token"))?;

        let roles = roles_str
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let permissions = permissions_str
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        Ok(Principal {
            id,
            username,
            roles,
            permissions,
        })
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

pub fn get_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
