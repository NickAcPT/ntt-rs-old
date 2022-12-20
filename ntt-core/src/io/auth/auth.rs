use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    TokenUrl,
};
use once_cell::sync::Lazy;
use std::borrow::Cow;

use crate::errors::NttCoreResult;
static GITHUB_AUTHORIZE_URL: Lazy<AuthUrl> = Lazy::new(|| {
    AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL")
});
static GITHUB_TOKEN_URL: Lazy<TokenUrl> = Lazy::new(|| {
    TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL")
});

pub struct NttAuthState {
    client: BasicClient,
    pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub auth_url: Url,
}

impl NttAuthState {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> NttCoreResult<Self> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            GITHUB_AUTHORIZE_URL.clone(),
            Some(GITHUB_TOKEN_URL.clone()),
        );

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let redirect = RedirectUrl::new(redirect_uri)?;
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .set_redirect_uri(Cow::Owned(redirect))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();

        Ok(NttAuthState {
            client,
            pkce_verifier,
            auth_url,
            csrf_token,
        })
    }
}
