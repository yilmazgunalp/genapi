pub const SCHEMA_TEMPLATE: &'static str = "
table! {
    {{name}}s (id) {
        id -> Integer,
        {{#fields}}
        {{name}} -> {{typ}},
        {{/fields}}
    }
}";

pub const SQL_DOWN_TEMPLATE: &'static str = "DROP TABLE {{name}}s";
pub const SQL_UP_TEMPLATE: &'static str = "CREATE TABLE {{name}}s (
    id SERIAL PRIMARY KEY,\
    {{#fields}}
    {{name}} {{typ}}\
    {{/fields}}
  )";

pub const MODELS_TEMPLATE: &'static str = "use crate::schema::*;
use serde::*;
   genapi_macro::create_model!(Record {
     name: \"{{name}}\", 
     fields: [ {{#fields}} 
    Field {
      name: \"{{name}}\",
      typ: \"{{typ}}\",
    },
        {{/fields}}],});\n

";

pub const ENDPOINTS_TEMPLATE: &'static str = "use diesel::prelude::*;
use crate::schema::{{name}}s;
use crate::models::*;
use crate::db;
use rocket::State;
use rocket_contrib::json::Json;

   genapi_macro::get_endpoint!(Record {
     name: \"{{name}}\", 
     fields: [ {{#fields}} 
    Field {
      name: \"{{name}}\",
      typ: \"{{typ}}\",
    },
        {{/fields}}],});\n

 genapi_macro::post_endpoint!(Record {
  name: \"{{name}}\", 
  fields: [ {{#fields}} 
 Field {
   name: \"{{name}}\",
   typ: \"{{typ}}\",
 },
     {{/fields}}],});\n

genapi_macro::delete_endpoint!(Record {
  name: \"{{name}}\", 
  fields: [ {{#fields}} 
 Field {
   name: \"{{name}}\",
   typ: \"{{typ}}\",
 },
     {{/fields}}],});\n

genapi_macro::put_endpoint!(Record {
  name: \"{{name}}\", 
  fields: [ {{#fields}} 
 Field {
   name: \"{{name}}\",
   typ: \"{{typ}}\",
 },
     {{/fields}}],});\n

";

pub const MAIN_TEMPLATE: &'static str = "#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod db;
pub mod endpoints;
pub mod models;
pub mod schema;

use rocket::config::{Config, Environment};

use std::env;

embed_migrations!();

fn main() {
    let connection = db::establish_connection();

    match embedded_migrations::run(&connection) {
        Ok(_) => println!(\"yahoo heeya\"),
        Err(e) => eprintln!(\"Oh noes, we don't know which era we're in! :( \n  {}\", e),
    }

    let port = env::var(\"PORT\")
        .unwrap()
        .parse::<u16>()
        .expect(\"$PORT must be set\");

    let config = Config::build(Environment::Production)
        .port(port)
        .finalize()
        .expect(\"Configuration error\");

    rocket::custom(config)
        .manage(db::establish_connection_pool())
        .mount(
            \"/\",
            routes![endpoints::list, endpoints::new, endpoints::update, endpoints::delete,],
        )
        .launch();
}
";

/*  Check what can be done with ramhorns like using expressions
otherwise in order to use mustache I need to create another data type which can be used instead
to write things like String, bool ...*/

// on second thought I can pass another field in Record struct for String bool etc
// but I'll check if I can use some rust closures and impl blocks(maybe by writing a derive macro) to convert `bike` to `Bike` and `NewBike`

//ok so.., I already had the solution! I need to write another function like macro similar to genapi::create where I take a Record ExprStruct which will generate the required Bike and NewBike structs.

// now question is Can I just pass Idents to the macro and create the Struct like that
// this depends on how dynamic the structure of the struct will be I guess...

// #[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
// #[table_name=\"{{name}}s\"]
// pub struct NewBike {
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }"
