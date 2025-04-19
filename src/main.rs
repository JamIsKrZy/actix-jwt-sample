use actix_web::{middleware, App, HttpServer};

mod app_state;
mod models;
mod routes;
mod auth;


#[actix_web::main]
async fn main() -> std::io::Result<()>{

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = app_state::DbDummy::init_app_data();
    let app_state = app_state::AppState::default();

    HttpServer::new(move ||{
        App::new()
            .app_data(db.clone())
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(routes::config)
    })
    .bind(("127.0.0.1",8080))?
    .workers(2)
    .run()
    .await
}
