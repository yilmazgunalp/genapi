#[derive(Debug)] 
pub struct Endpoint<'a> { 
    path: &'a str,
    method: Method,
    response: Response<'a>,
}



#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

#[derive(Debug)]
pub struct Response<'a> {
    pub content_type: ContentType,
    pub body: &'a str,
}

#[derive(Debug)]
pub enum ContentType {
    JSON,
    TEXT,
}

