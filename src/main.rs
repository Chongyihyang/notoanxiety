#![allow(non_snake_case )]
mod create;
mod index;
mod show;
mod macros;
mod model;
mod auth;
mod jwt;
mod constants;
mod delete;

use actix_files::Files;
use actix_web::web::ServiceConfig;
use auth::{login, login_path, logout_path};
use create::{create_path, create_post};
use delete::delete_path;
use index::index_path;
use show::show_path;
use shuttle_actix_web::ShuttleActixWeb;


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
        cfg.service(show_path);
        cfg.service(delete_path);
    };

    Ok(config.into())
}
