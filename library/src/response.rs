use poem::{http::StatusCode, IntoResponse};
use serde::Serialize;
use std::fmt::Debug;
#[derive(Debug, Serialize, Default)]
pub struct Response<T> {
    pub code: Option<u32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

// 允许无条件递归
#[allow(unconditional_recursion)]
impl<T> IntoResponse for Response<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> poem::Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(val) => val,
            Err(err) => {
                return poem::Response::from((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
            }
        };
        let copy_json = json_string.clone();
        let mut result = json_string.into_response();
        result.extensions_mut().insert(copy_json);
        result
    }
}

impl<T: Serialize> Response<T> {
    pub fn data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }

    pub fn error(err: &str) -> Self {
        Self {
            code: Some(400),
            data: None,
            msg: Some(err.to_string()),
        }
    }

    pub fn messsage(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }

    pub fn success(data: T, msg: &str) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
