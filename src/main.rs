
#![feature(proc_macro_hygiene, decl_macro)]
mod subcommands;
use ramhorns::{Template, Content};

use std::ffi::OsString;
use rocket_contrib::json::Json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

use subcommands::compile_api;
use subcommands::{create_tmp_file,RcronError};


use::std::io::Write;

#[post("/api/gen", data = "<endpoint>")]
fn genome(endpoint: Json<Endpoint>) -> Result<NamedFile, RcronError> {


    let epis: Epis = Epis {
        epis: vec!(endpoint.0)
    };


    let src_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_api/");
    let source = "
fn main() {
    {{#epis}}genapi_macro::create!(Endpoint {path: \"{{path}}\", method: Method::GET, response: {{#response}}Response {
        content_type: ContentType::TEXT,
        body: \"{{body}}\"
    }{{/response}}});\n{{/epis}}
    genapi_macro::ignite!([{{#epis}}{{path}},{{/epis}}]);
}";



let tpl = Template::new(source).unwrap();
let rendered = tpl.render(&epis);
println!("{}",rendered);

let imports = vec!("#![feature(proc_macro_hygiene, decl_macro)]\n".as_bytes(), "#[macro_use] extern crate rocket;\n".as_bytes(),"mod types;\nuse types::{ContentType,Response};\n".as_bytes());
 let (mut tmp_file, tmp_file_path) = create_tmp_file()?;
    
    tmp_file.write(&imports[0])?;
    tmp_file.write(&imports[1])?;
    tmp_file.write(&imports[2])?;
    tmp_file.write(&rendered.as_bytes())?;
    compile_api(&src_path);
    let path =  OsString::from("/home/yg/ygprojects/genapi/output_binary/hello_world");;

    NamedFile::open(&path).map_err(|e| RcronError::default())
}

#[allow(unused_imports)]
fn main()  {       

    rocket::ignite().mount("/", routes![genome]).launch();
}

type  Met = Method;

#[derive(Debug,Content,Deserialize)] 
struct Endpoint<'a> { 
    path: &'a str,
    method: MethodS<'a>,
    response: Response<'a>,
}

#[derive(Debug,Content,Deserialize)]
struct Response<'a> {
    content_type: ContentType,
    body: &'a str,
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
}
#[derive(Debug,Content,Deserialize)] 
struct  MethodS<'a> {
    method: &'a str
}

#[derive(Debug,Deserialize)]
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
    epis: Vec<Endpoint<'a>>
}