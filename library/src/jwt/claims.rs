use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use poem::web::TypedHeader;
use poem::web::headers::Authorization;
use poem::web::headers::authorization::Bearer;
use serde::{Deserialize, Serialize};
use poem::{
    Result, http::StatusCode, Error,FromRequest,
    Request,RequestBody,
};

const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "";

#[derive(Debug,Clone, Default, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub user_id: u32,
    pub exp: i64,
}

impl Claims {
    pub fn new(username: String, user_id:u32) -> Self {
        Self {
            username,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
            user_id,
        }
    }
}

pub fn create_jwt(claims: Claims) -> Result<String> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| Error::from_string("Token create error", StatusCode::INTERNAL_SERVER_ERROR)) 
}

pub fn decode_jwt(token: &str) -> Result<Claims> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| {
            data.claims
        })
        .map_err(|err| {
            Error::from_string(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        }) 
        
}



#[poem::async_trait]
impl <'a> FromRequest<'a> for Claims {
    // 为用户信息实现from request trait
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self>{
        let token_val = get_bear_token(req).await?;
        let token_data = match decode_jwt(&token_val){
            Ok(token) => token,
            Err(e) => return Err(e),
        };
        Ok(token_data)
    }
    async fn from_request_without_body(req: &'a Request) -> Result<Self> {
        Self::from_request(req, &mut Default::default()).await
    }
}

pub async fn get_bear_token(req: &Request) -> Result<String> {
    let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_without_body(req)
        .await
        .map_err(|_| Error::from_string("InvalidToken", StatusCode::BAD_REQUEST))?;
    // 解码用户数据
    let bearer_data = bearer.token();
    Ok(bearer_data.to_string())
}