#[macro_export]
macro_rules! render_template {
    (
        $tmpl:expr
        $(, $key:ident $(=> $value:expr)?)* $(,)?
    ) => {{
        use std::fs::File;
        use std::io::prelude::*;
        use minijinja::render;
        use actix_web::{http::StatusCode, HttpResponse};
        let mut html = String::new();
        let mut file = File::open($tmpl).expect("no file is found");
        file.read_to_string(&mut html).expect("cannot read string");
        HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(
            render!(&html, $($key $(=> $value)? ,)*)
        )
    }};

}

#[macro_export]
macro_rules! get_items {    
    (
        $conn: expr,
        $statement: expr
    ) => {{
        $conn
        .prepare($statement)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    }};
}

#[macro_export]
macro_rules! f_string {
    ($($tokens:tt)*) => {
        format!($($tokens)*)
    };
}

use actix_web::cookie::{Cookie, SameSite};
pub fn build_cookie<'a>(name: & 'a str, value: String) -> Cookie<'a> {
    Cookie::build(name, value)
    .http_only(true)
    .same_site(SameSite::None)
    .finish()
}
