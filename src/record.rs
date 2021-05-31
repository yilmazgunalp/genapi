use serde::*;
use ramhorns::{Template, Content};
#[derive(Debug,Serialize,Deserialize, Content)]
pub struct Record {
    pub name: String,
    pub fields: Vec<Field>,
}
#[derive(Debug,Serialize,Deserialize, Content)]
pub struct Field {
    name: String,
    typ: String,
    // TODO add other fields like optional, default etc.. 
}
#[derive(Debug,Serialize,Deserialize)]
pub enum Column {
   VarChar,
   Text,
   Bool,
   //TODO add other types
}

impl Content for Column {
    fn capacity_hint(&self, _tpl: &Template) -> usize {
            32
    }
}