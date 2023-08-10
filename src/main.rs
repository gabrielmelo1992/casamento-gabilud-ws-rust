mod app;

use actix_web::{
    web,
    App,
    HttpServer,
};
use std::env;
use app::components::mongo::Mongo;
use app::models::app::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let mongo_db = Mongo::init().await;
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");
    println!("🚀 Binding server to {}", bind_address);
    return HttpServer::new(move
            || App::new()
                    .app_data(web::Data::new(AppState {
                        app_name: env::var("APP_NAME").expect("APP_NAME is not set"),
                        mongo: mongo_db.clone(),
                    }))
                    .route("/", web::get().to(app::index))
                    .route("/invite", web::post().to(app::routes::invite::invite))
                    // .service(
                    //     web::scope("/invite")
                    //         .route("/login", web::post().to(app::routes::auth::login::login)
                    //     )
                    // )
        )
        .bind(bind_address)?
        .run()
        .await;
}