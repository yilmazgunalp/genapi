use ramhorns::Content;
use serde::*;
#[derive(Debug, Serialize, Deserialize, Content)]
pub struct Record {
    pub name: String,
    pub fields: Vec<Field>,
}
#[derive(Debug, Serialize, Deserialize, Content)]
pub struct Field {
    pub name: String,
    pub typ: String,
}
