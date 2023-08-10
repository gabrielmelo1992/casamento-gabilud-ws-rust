use crate::app::utils::Response;
use crate::app::models::app::AppState;
// use crate::db::{new_pool, DbExecutor};
// use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    web,
    HttpResponse,
    Responder,
};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Invite {
    name: String,
}

pub async fn invite(body: web::Json<Invite>, data: web::Data<AppState>) -> impl Responder {
    const MESSAGE: &str = "Invite";
    if body.name == "" {
        return HttpResponse::BadRequest().json(Response {
            status: "nok".to_string(),
            message: "Name is required".to_string(),
        });
    }
    match data.mongo.insert_invite(body.name.clone()).await {
        Ok(_) => println!("🚀 Inserted invite {}", body.name),
        Err(_) => {
            println!("🔥 Failed to insert invite {}", body.name);
            return HttpResponse::BadRequest().json(Response {
                status: "nok".to_string(),
                message: "Failed to insert invite".to_string(),
            });
        }
    }
    
    let response_json: &Response = &Response {
        status: "ok".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}