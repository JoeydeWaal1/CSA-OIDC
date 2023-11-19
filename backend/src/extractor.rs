use axum::{extract::FromRequestParts, async_trait, http::request::Parts, headers::{authorization::Bearer, Authorization}, TypedHeader};
use serde::Deserialize;
use axum::http::StatusCode;

#[derive(Debug,Deserialize)]
pub struct User {
    pub nickname: String,
    pub name: String,
    pub email: String
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state:&S) -> Result<Self,Self::Rejection>
    {
        //  src/extractor.rs
        // access_token uit request halen van SPA,
        // anders afwijzen
        let Ok(access_token) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await else {
            return Err(StatusCode::UNAUTHORIZED);
        };

        // authority uit env variable halen
        let authority = std::env::var("AUTHORITY").expect("AUTHORITY env var bestaat niet");
        println!("{authority}");

        // claims halen van access token
        let Ok(request) = reqwest::Client::new()
            .get(format!("https://{authority}/userinfo"))
            .header("Authorization", format!("Bearer {}",access_token.token()))
            .send().await else {
                return Err(StatusCode::UNAUTHORIZED);
            };

        // body uit response halen
        let raw_text = request.text().await.map_err(|_| StatusCode::UNAUTHORIZED)?;

        // body omzetten naar een struct die de API kan gebruiken
        let user:User = serde_json::from_str(&raw_text).map_err(|_| StatusCode::UNAUTHORIZED)?;
        // let user = User { nickname: "joey".into(), name: "joey".into(), email: "sjoey".into() };
        Ok(user)
    }
}
