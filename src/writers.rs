use std::{ffi::OsString, fs::File, io::Write};

use ramhorns::Template;

use crate::templates::*;
use crate::{compile::GenapiError, record::Record};

pub fn write_schema(record: &Record) -> Result<(), GenapiError> {
    let source = SCHEMA_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/schema.rs")?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_down_migration(record: &Record) -> Result<(), GenapiError> {
    let source = SQL_DOWN_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/migrations/01_create/down.sql")?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_up_migration(record: &Record) -> Result<(), GenapiError> {
    let source = SQL_UP_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/migrations/01_create/up.sql")?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_models(record: &Record) -> Result<(), GenapiError> {
    let source = MODELS_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/models.rs")?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn write_endpoints(record: &Record) -> Result<(), GenapiError> {
    let source = ENDPOINTS_TEMPLATE;
    let tpl = Template::new(source).unwrap();
    let rendered = tpl.render(record);
    let (mut tmp_file, _tmp_file_path) = create_tmp_file("/src/endpoints.rs")?;
    tmp_file.write(&rendered.as_bytes())?;
    Ok(())
}

pub fn create_tmp_file(filename: &str) -> Result<(File, OsString), GenapiError> {
    let tmp_file_path: OsString =
        OsString::from(format!("/home/yg/ygprojects/genapi/output_api{}", filename));
    let tmp_file: File = File::create(&tmp_file_path).expect("Failed at creating tmp file!");

    Ok((tmp_file, tmp_file_path))
}
