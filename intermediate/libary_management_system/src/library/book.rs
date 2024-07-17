
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub available: bool
}

impl Book {
    pub fn new(title:&str, author:&str, isbn:&str) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            available: true
        }
    }
}