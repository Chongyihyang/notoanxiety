use actix_web::{get, post, web::{Form, Redirect}, HttpRequest, HttpResponse, Either};
use sqlite::Connection;

use crate::{model::PostInfo, render_template, f_string, jwt::decode_jwt};

#[get("/create")]
pub async fn create_path(msg: HttpRequest) -> Either<HttpResponse, Redirect>{
    if let None = msg.cookie("token") {
        return Either::Right(Redirect::to("/login"))
    }
    let x = msg.cookie("token").unwrap();
    if let Err(_) = decode_jwt(x.value().to_owned()) {
        return Either::Left(HttpResponse::Unauthorized().finish())
    }
    Either::Left(render_template!("templates/create.html"))
}

#[post("/create")]
pub async fn create_post(msg: HttpRequest, p: Form<PostInfo>) -> Either<HttpResponse, Redirect> {
    if let None = msg.cookie("token") {
        return Either::Right(Redirect::to("/login").see_other() )
    }
    let x = msg.cookie("token").unwrap();
    if let Err(_) = decode_jwt(x.value().to_owned()) {
        return Either::Left(HttpResponse::Unauthorized().finish())
    }
    let conn = Connection::open("database.db").unwrap();
    let statement = f_string!("INSERT INTO posts (title, date, description) VALUES ('{}', '{}', '{}')", p.title, p.date, p.description);
    conn.execute(statement).unwrap();
    Either::Right(Redirect::to("/").see_other())
}