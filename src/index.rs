use actix_web::{get, web::Redirect, Either, HttpRequest, HttpResponse};
use sqlite::Connection;
use crate::{get_items, jwt::decode_jwt, model::PostInfo, render_template};


#[get("/")]
pub async fn index_path(msg: HttpRequest)  -> Either<HttpResponse, Redirect> {
    if let None = msg.cookie("token") {
        return Either::Right(Redirect::to("/login"))
    }
    let x = msg.cookie("token").unwrap();
    if let Err(_) = decode_jwt(x.value().to_owned()) {
        return Either::Right(Redirect::to("/login"))
    }
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
    order_posts.reverse();
    Either::Left(render_template!("templates/index.html", posts => order_posts))
}