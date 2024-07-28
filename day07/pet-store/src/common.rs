// use mongodb::bson::binary::Result;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use rocket_db_pools::mongodb;
use rocket_db_pools::Database;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone)]
pub struct RequestId(pub String);

#[derive(Database)]
#[database("pet_store")]
pub struct DbClient(mongodb::Client);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestId {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let request_id = Uuid::new_v4().to_string();
        Outcome::Success(request.local_cache(|| RequestId(request_id)).clone())
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum AppErrorCode {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
    ServerError,
}

#[derive(Deserialize, Serialize)]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}
impl AppError {
    pub fn bad_request(msg: &str) -> Self {
        Self {
            code: AppErrorCode::BadRequest,
            message: msg.to_owned(),
        }
    }
    pub fn forbidden() -> Self {
        Self {
            code: AppErrorCode::Forbidden,
            message: "you are not allowed to use this api".to_owned(),
        }
    }

    pub fn not_found(msg: &str) -> Self {
        Self {
            code: AppErrorCode::NotFound,
            message: msg.to_owned(),
        }
    }
    pub fn server_error() -> Self {
        Self {
            code: AppErrorCode::ServerError,
            message: "server error occur! please contact your customer service".to_owned(),
        }
    }
    pub fn conflict(msg: &str) -> Self {
        Self {
            code: AppErrorCode::Conflict,
            message: msg.to_owned(),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(value: mongodb::error::Error) -> Self {
        log::error!("mongodb error: {:?}", &value);
        Self::server_error()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize, Debug, Clone)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct PagedResponse<T> {
    pub total: u64,
    pub items: Vec<T>,
}
