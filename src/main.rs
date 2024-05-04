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
use actix_web::{HttpServer, App, web::ServiceConfig};
use auth::{login, login_path, logout_path, logout_to_login};
use create::{create_path, create_post};
use index::index_path;
use actix_web::cookie::Key;
use show::show_path;
use shuttle_actix_web::ShuttleActixWeb;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
	    CookieSessionStore::default(), Key::from(&[0; 64])
    )
	.build()
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>{
    let config = move |cfg: &mut ServiceConfig| {
        // set up your service here, e.g.:
        cfg.service(login_path);
        cfg.service(login);
        cfg.service(index_path);
        cfg.service(Files::new("/static", "./static"));
        cfg.service(create_path);
        cfg.service(create_post);
        cfg.service(logout_path);
        cfg.service(logout_to_login);
        cfg.service(show_path);
    };

    Ok(config.into())
}
