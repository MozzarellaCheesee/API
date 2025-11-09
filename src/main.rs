mod db;
mod models;
mod schema;
mod structs;
mod error;
mod routes;

use actix_web::{
    web,
    App,HttpServer
};
use argon2::{Algorithm, Argon2, Params, Version};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use routes::post::*;
use db::establish_connection_pool;
use crate::structs::AppState;

pub type ConnPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection_pool().await;

    let params = match Params::new(19456, 4, 4, None){
        Ok(p) => p,
        Err(_) => return Err(
            std::io::Error::new(std::io::ErrorKind::Other, "Ошибка хэширования".to_string()))
    };

    let argon2 = Argon2::new(
        Algorithm::Argon2id, Version::V0x13, params);

    let app_state = AppState::new(pool, argon2);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(registry)
            .service(logout)
            .service(login)
            .service(confirm)
            .service(pings)

    })
        .bind(("0.0.0.0", 5002))?
        .run()
        .await
}



