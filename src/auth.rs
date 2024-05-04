use std::fs::File;
use std::io::prelude::*;
use actix_web::{get, post, web::{Form, Redirect}, Either, HttpResponse, Responder};
use sqlite::Connection;
use sha256::digest;
use actix_web::cookie::{Cookie, SameSite};
use minijinja::render;
use crate::model::PostInfo;

use crate::{f_string, get_items, jwt::encode_jwt, model::User, render_template};

#[post("/login")]
pub async fn login(p: Form<User>) -> Either<HttpResponse, Redirect> {

    let conn = Connection::open("database.db").unwrap();
    let statement = 
    f_string!("SELECT password FROM users WHERE username = '{}'", p.username);
    let mut hashed = vec![];
    for row in get_items!(conn, statement) {
        hashed.push(row.read::<&str, _>("password").to_owned())
    }
    if let None = hashed.iter().nth(0) {
        return Either::Right(Redirect::to("/login").see_other())
    }
    return if hashed.iter().nth(0).unwrap() == &digest(p.password.clone()) {
        let c = Cookie::build("token", encode_jwt(p.username.clone()).unwrap())
        .http_only(true)
        .same_site(SameSite::None)
        .finish();
        let conn = Connection::open("database.db").unwrap();
        let mut order_posts = vec![];
        for row in get_items!(conn, "SELECT * FROM posts") {
            order_posts.push((row.read::<i64, _>("id").to_owned(),
                PostInfo::new(
                row.read::<&str, _>("title").to_owned(),
                row.read::<&str, _>("date").to_owned(),
                row.read::<&str, _>("description").to_owned(),
            )))
        }
        let mut html = String::new();
        let mut file = File::open("templates/index.html").expect("no file is found");
        file.read_to_string(&mut html).expect("cannot read string");
        Either::Left(HttpResponse::Ok().cookie(c).body(render!(&html, posts => order_posts)))
    } else {
        Either::Right(Redirect::to("/login").see_other())
    }

}

#[get("/logout")]
pub async fn logout_path() -> HttpResponse {
    let c = Cookie::build("token", "")
    .http_only(true)
    .same_site(SameSite::None)
    .finish();
    let mut html = String::new();
    let mut file = File::open("templates/login.html").expect("no file is found");
    file.read_to_string(&mut html).expect("cannot read string");
    HttpResponse::Ok().cookie(c).body(render!(&html))
}

#[post("/logout")]
pub async fn logout_to_login(p: Form<User>) -> Either<HttpResponse, Redirect>{

    let conn = Connection::open("database.db").unwrap();
    let statement = 
    f_string!("SELECT password FROM users WHERE username = '{}'", p.username);
    let mut hashed = vec![];
    for row in get_items!(conn, statement) {
        hashed.push(row.read::<&str, _>("password").to_owned())
    }
    if let None = hashed.iter().nth(0) {
        return Either::Right(Redirect::to("/login").see_other())
    }
    return if hashed.iter().nth(0).unwrap() == &digest(p.password.clone()) {
        let c = Cookie::build("token", encode_jwt(p.username.clone()).unwrap())
        .domain("127.0.0.1")
        .http_only(true)
        .same_site(SameSite::None)
        .finish();
        let conn = Connection::open("database.db").unwrap();
        let mut order_posts = vec![];
        for row in get_items!(conn, "SELECT * FROM posts") {
            order_posts.push((row.read::<i64, _>("id").to_owned(),
                PostInfo::new(
                row.read::<&str, _>("title").to_owned(),
                row.read::<&str, _>("date").to_owned(),
                row.read::<&str, _>("description").to_owned(),
            )))
        }
        let mut html = String::new();
        let mut file = File::open("templates/index.html").expect("no file is found");
        file.read_to_string(&mut html).expect("cannot read string");
        Either::Left(HttpResponse::Ok().cookie(c).body(render!(&html, posts => order_posts)))
    } else {
        Either::Right(Redirect::to("/login").see_other())
    }
}

#[get("/login")]
pub async fn login_path() -> impl Responder {
    render_template!("templates/login.html")
}