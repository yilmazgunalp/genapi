use super::record::{Field, Record};

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn convert_fields_to_sql(a: &Record) -> Record {
    let mut b: Vec<Field> = a
        .fields
        .iter()
        .map(|f| {
            if String::from("Text") == f.typ {
                return Field {
                    name: String::from(&f.name),
                    typ: String::from("VARCHAR NOT NULL,"),
                };
            } else {
                return Field {
                    name: String::from(&f.name),
                    typ: String::from("BOOLEAN NOT NULL DEFAULT 'f',"),
                };
            }
        })
        .collect();
    let l = &b.len();
    b.iter_mut().nth(l - 1).map(|f| f.typ.pop());
    Record {
        name: String::from(&a.name),
        fields: b,
    }
}
