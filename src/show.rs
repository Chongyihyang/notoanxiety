use actix_web::{ get, web::{Redirect, Path}, Either, HttpRequest, HttpResponse};
use sqlite::Connection;

use crate::{f_string, get_items, jwt::decode_jwt, model::PostInfo, render_template};

#[get("/show/{id}")]
pub async fn show_path(post_number : Path<usize>, msg: HttpRequest) -> Either<HttpResponse, Redirect> {
    if let None = msg.cookie("token") {
        return Either::Right(Redirect::to("/login"))
    }
    let x = msg.cookie("token").unwrap();
    if let Err(_) = decode_jwt(x.value().to_owned()) {
        return Either::Right(Redirect::to("/login"))
    }
    let post_number = post_number.into_inner();
    let conn = Connection::open("database.db").unwrap();
    let mut order_posts = vec![];
    for row in get_items!(conn, f_string!("SELECT * FROM posts where id = {}", post_number)) {
        order_posts.push(
            PostInfo::new(
            row.read::<&str, _>("title").to_owned(),
            row.read::<&str, _>("date").to_owned(),
            row.read::<&str, _>("description").to_owned(),
        ))
    }
    Either::Left(render_template!("templates/show.html", item => order_posts[0], id => post_number))
}