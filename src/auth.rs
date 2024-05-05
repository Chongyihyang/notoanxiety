use actix_web::{get, http::{header, StatusCode, header::HeaderValue}, post, web::{Form, Redirect}, Either, HttpResponse, Responder};
use sqlite::Connection;
use sha256::digest;
use actix_web::cookie::{Cookie, SameSite};
use minijinja::render;

use crate::{f_string, get_items, jwt::encode_jwt, macros::build_cookie, model::User, render_template};

#[get("/login")]
pub async fn login_path() -> impl Responder {
    render_template!("templates/login.html")
}

#[post("/login")]
pub async fn login(p: Form<User>) -> Either<impl Responder, Redirect> {

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
        let c = build_cookie("token", encode_jwt(p.username.clone()).unwrap());
        let mut response = HttpResponse::Ok();
        response.cookie(c);
        response.status(StatusCode::SEE_OTHER)
        .append_header((
            header::LOCATION,
            HeaderValue::from_static("/"),
        ))
        ;
        Either::Left(response)
    } else {
        Either::Right(Redirect::to("/login").see_other())
    }

}

#[get("/logout")]
pub async fn logout_path() -> impl Responder {
    let c = Cookie::build("token", "")
    .http_only(true)
    .same_site(SameSite::None)
    .finish();
    let mut response = HttpResponse::Ok();
    response.cookie(c);
    response.status(StatusCode::SEE_OTHER)
    .append_header((
        header::LOCATION,
        HeaderValue::from_static("/"),
    ))
    ;
    response
}