#![allow(non_snake_case )]
mod create;
mod index;
mod show;
mod macros;
mod model;
mod auth;
mod jwt;
mod constants;

use actix_files::Files;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpServer, App};
use auth::{login, login_path, logout_path, logout_to_login};
use create::{create_path, create_post};
use index::index_path;
use actix_web::cookie::Key;
use show::show_path;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
	    CookieSessionStore::default(), Key::from(&[0; 64])
    )
	.build()
}


#[tokio::main]
async fn main() {
    let app = || {
        App::new()
        .wrap(session_middleware())
        .service(login_path)
        .service(login)
        .service(index_path)
        .service(Files::new("/static", "./static"))
        .service(create_path)
        .service(create_post)
        .service(logout_path)
        .service(logout_to_login)
        .service(show_path)
    };

    let _ = HttpServer::new(app)
    .bind(("127.0.0.1", 5000))
    .unwrap()
    .run()
    .await;
}
