mod subcommands;
use ramhorns::{Template, Content};
use ramhorns::encoding::Encoder;
use std::ffi::OsString;

use subcommands::compile_api;
use subcommands::{create_tmp_file,RcronError};


use::std::io::Write;
#[allow(unused_imports)]
fn main() -> Result<(), RcronError>  {
    
    let src_path: OsString = OsString::from("/home/yg/ygprojects/genapi/output_api/");

    let epi = Endpoint  {

        method: MethodS {method: "geat"},
        path: "cleanme",
        response: Response {
            content_type: ContentType::TEXT,
            body: "hulalla"
        }
    };

    let epi2 = Endpoint  {
        method: MethodS {method: "poast"},
        path: "merhabalar",
        response: Response {
            content_type: ContentType::TEXT,
            body: "dunya dunya"
        }
    };

    let epis: Epis = Epis {
        epis: vec!(epi, epi2)
    };

     
    // Standard Mustache action here for mutiple endpoints
// let source = "{{#epis}}#[{{method}}(\"/{{path}}\")]
// fn {{path}}() -> &'static str {
//     \"{{#response}}{{body}}{{/response}}\"
// }{{/epis}}
// fn main() {
//     S

// Standard Mustache action here single endpoints

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
Ok(())
}

type  Met = Method;

#[derive(Debug,Content)] 
struct Endpoint<'a> { 
    path: &'a str,
    method: MethodS<'a>,
    response: Response<'a>,
}

#[derive(Debug,Content)]
struct Response<'a> {
    content_type: ContentType,
    body: &'a str,
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
}
#[derive(Debug,Content)] 
struct  MethodS<'a> {
    method: &'a str
}

#[derive(Debug)]
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