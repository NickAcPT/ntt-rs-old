use oauth2::basic::BasicClient;
use oauth2::url::Url;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, TokenUrl,
};

use crate::errors::NttCoreResult;

pub struct NttAuthState {
    client: BasicClient,
    pkce_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub auth_url: Url,
}

impl NttAuthState {
    const GITHUB_AUTHORIZE_URL: &'static str = "https://github.com/login/oauth/authorize";
    const GITHUB_TOKEN_URL: &'static str = "https://github.com/login/oauth/access_token";

    pub fn new(client_id: String, client_secret: String) -> NttCoreResult<Self> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(Self::GITHUB_AUTHORIZE_URL.to_string())?,
            Some(TokenUrl::new(Self::GITHUB_TOKEN_URL.to_string())?),
        );

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
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
