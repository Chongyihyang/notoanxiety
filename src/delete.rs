use actix_web::{get, web::{Path, Redirect}, HttpRequest,};
use sqlite::Connection;
use crate::{jwt::decode_jwt,  f_string};

#[get("/delete/{id}")]
pub async fn delete_path(post_number : Path<usize>, msg: HttpRequest) -> Redirect{
    if let None = msg.cookie("token"){
        return Redirect::to("/login")
    }
    let x = msg.cookie("token").unwrap();
    if let Err(_) = decode_jwt(x.value().to_owned()) {
        return Redirect::to("/login")
    }
    let conn = Connection::open("database.db").unwrap();
    let statement = f_string!("DELETE FROM posts WHERE id = {}", post_number);
    match conn.execute(statement) {
        Ok(_) => {Redirect::to("/")},
        Err(_) => {Redirect::to("/")}
    }
}