use crate::app::utils::Response;
// use crate::db::{new_pool, DbExecutor};
// use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    HttpResponse,
    Responder,
};

pub async fn invite() -> impl Responder {
    const MESSAGE: &str = "Invite";

    let response_json: &Response = &Response {
        status: "ok".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}