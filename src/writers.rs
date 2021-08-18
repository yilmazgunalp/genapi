use std::{
    ffi::OsString,
    fs::{DirBuilder, File, OpenOptions},
    io::Write,
};

use ramhorns::Template;

use crate::templates::*;
use crate::{compile::GenapiError, record::Record};

pub fn write_schema(record: &Record) -> Result<(), GenapiError> {
    let source = SCHEMA_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/schema.rs", None)?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_down_migration(record: &Record, now: &str) -> Result<(), GenapiError> {
    let source = SQL_DOWN_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("down.sql", Some(now))?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_up_migration(fields: &Record, now: &str) -> Result<(), GenapiError> {
    let source = SQL_UP_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(fields);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("up.sql", Some(now))?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_models(record: &Record) -> Result<(), GenapiError> {
    let source = MODELS_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/models.rs", None)?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_endpoints(record: &Record) -> Result<(), GenapiError> {
    let source = ENDPOINTS_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/endpoints.rs", None)?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_main(record: &Record) -> Result<(), GenapiError> {
    let source = MAIN_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/main.rs", None)?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn create_tmp_file(filename: &str, dir: Option<&str>) -> Result<(File, OsString), GenapiError> {
    let tmp_file_path: OsString = match dir {
        Some(dirname) => match DirBuilder::new().recursive(true).create(format!(
            "/home/yg/ygprojects/genapi/output_api/migrations/{}/",
            dirname
        )) {
            Ok(_) => OsString::from(format!(
                "/home/yg/ygprojects/genapi/output_api/migrations/{}/{}",
                dirname, filename
            )),
            Err(e) => panic!("Could not create the directory: {}", e),
        },
        None => OsString::from(format!("/home/yg/ygprojects/genapi/output_api{}", filename)),
    };
    let tmp_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&tmp_file_path)
        .expect(&format!("Failed at creating tmp file!{}", filename));

    Ok((tmp_file, tmp_file_path))
}
