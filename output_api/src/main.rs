#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod types;
use types::{ContentType,Response};

fn main() {
    genapi_macro::create!(Endpoint {path: "cleanme", method: Method::GET, response: Response {
        content_type: ContentType::TEXT,
        body: "hulalla"
    }});
genapi_macro::create!(Endpoint {path: "merhabalar", method: Method::GET, response: Response {
        content_type: ContentType::TEXT,
        body: "dunya dunya"
    }});

    genapi_macro::ignite!([cleanme,merhabalar,]);
}