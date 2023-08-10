mod utils;
pub mod routes;
pub mod components;
pub mod models;

// use crate::db::{new_pool, DbExecutor};
// use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    HttpResponse,
    Responder,
};

pub async fn index() -> impl Responder {
    const MESSAGE: &str = "Server is running successfully";

    let response_json: &utils::Response = &utils::Response {
        status: "ok".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}