#![feature(proc_macro_hygiene, decl_macro)]
mod compile;

use rocket_contrib::json::Json;
use std::{ffi::OsString, time::SystemTime};
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::response::NamedFile;

use compile::compile_api;
use compile::GenapiError;

mod record;
mod templates;
mod writers;
use record::Record;
mod utils;

#[post("/api/gen", data = "<record>")]
fn genome(record: Json<Record>) -> Result<NamedFile, GenapiError> {
    let rec = record.into_inner();
    println!("{:?}", rec);
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let dir = format!("{}_create", now);

    writers::write_schema(&rec);
    writers::write_down_migration(&rec, &dir);
    writers::write_up_migration(&utils::convert_fields_to_sql(&rec), &dir);
    writers::write_models(&rec);
    writers::write_endpoints(&rec);
    writers::write_main(&rec);

    let src_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_api/");
    compile_api(&src_path);
    let path = OsString::from("/home/yg/ygprojects/genapi/output_binary/hello_world");

    NamedFile::open(&path).map_err(|e| GenapiError::default())
}

#[allow(unused_imports)]
fn main() {
    rocket::ignite().mount("/", routes![genome]).launch();
}
