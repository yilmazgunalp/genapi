#![feature(proc_macro_hygiene, decl_macro)]
mod compile;
use ramhorns::{Content, Template};

use rocket_contrib::json::Json;
use std::ffi::OsString;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
use rocket::response::status::NotFound;
use rocket::response::NamedFile;

use compile::compile_api;
use compile::GenapiError;

mod record;
mod templates;
mod writers;
use record::{Field, Record};
mod utils;

use ::std::io::Write;

#[post("/api/gen", data = "<record>")]
fn genome(record: Json<Record>) -> Result<NamedFile, GenapiError> {
    let rec = record.into_inner();
    println!("{:?}", rec);

    // let r = Record {
    //     name: "ruya".to_owned(),
    //     fields: vec![
    //         Field {
    //             name: "funda".to_owned(),
    //             typ: "Text",
    //         },
    //         Field {
    //             name: "halime",
    //             typ: "VarChar",
    //         },
    //     ],
    // };

    writers::write_schema(&rec);
    writers::write_down_migration(&rec);
    writers::write_up_migration(&rec);
    writers::write_models(&rec);

    //     let epis: Epis = Epis {
    //         epis: vec!(endpoint.0)
    //     };

    let src_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_api/");
    //     let source = "
    // fn main() {
    //     {{#epis}}genapi_macro::create!(Endpoint {path: \"{{path}}\", method: Method::GET, response: {{#response}}Response {
    //         content_type: ContentType::TEXT,
    //         body: \"{{body}}\"
    //     }{{/response}}});\n{{/epis}}
    //     genapi_macro::ignite!([{{#epis}}{{path}},{{/epis}}]);
    // }";

    // let tpl = Template::new(source).unwrap();
    // let rendered = tpl.render(&epis);
    // println!("{}",rendered);

    let imports = vec![
        "#![feature(proc_macro_hygiene, decl_macro)]\n".as_bytes(),
        "#[macro_use] extern crate rocket;\n".as_bytes(),
        "mod types;\nuse types::{ContentType,Response};\n".as_bytes(),
    ];
    let (mut tmp_file, tmp_file_path) = writers::create_tmp_file("main.rs")?;

    tmp_file.write(&imports[0])?;
    tmp_file.write(&imports[1])?;
    tmp_file.write(&imports[2])?;
    // tmp_file.write(&rendered.as_bytes())?;
    compile_api(&src_path);
    let path = OsString::from("/home/yg/ygprojects/genapi/output_binary/hello_world");

    NamedFile::open(&path).map_err(|e| GenapiError::default())
}

#[allow(unused_imports)]
fn main() {
    rocket::ignite().mount("/", routes![genome]).launch();
}

type Met = Method;

#[derive(Debug, Content, Deserialize)]
struct Endpoint<'a> {
    path: &'a str,
    method: MethodS<'a>,
    response: Response<'a>,
}

#[derive(Debug, Content, Deserialize)]
struct Response<'a> {
    content_type: ContentType,
    body: &'a str,
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
}
#[derive(Debug, Content, Deserialize)]
struct MethodS<'a> {
    method: &'a str,
}

#[derive(Debug, Deserialize)]
enum ContentType {
    JSON,
    TEXT,
}

impl Content for Method {
    fn capacity_hint(&self, _tpl: &Template) -> usize {
        32
    }
}

impl Content for ContentType {
    fn capacity_hint(&self, _tpl: &Template) -> usize {
        32
    }
}

#[derive(Content)]
struct Epis<'a> {
    epis: Vec<Endpoint<'a>>,
}
