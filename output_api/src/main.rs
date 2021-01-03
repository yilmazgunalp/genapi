#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod types;
use types::{ContentType,Response};

fn main() {
    genapi_macro::create!(Endpoint {path: "godeep", method: Method::GET, response: Response {
        content_type: ContentType::TEXT,
        body: "white horse"
    }});

    genapi_macro::ignite!([godeep,]);
}