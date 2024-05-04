use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref TOKEN: String = set_token();
}

pub fn set_token() -> String{
    dotenv().ok();
    env::var("TOKEN").unwrap()
}