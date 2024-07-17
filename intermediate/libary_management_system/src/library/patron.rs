
#[derive(Debug)]
pub struct Patron {
    pub name: String,
    pub borrowed_books_isbn: Vec<String>
}

impl Patron {   
    pub fn new(name: &str) -> Patron {
        Patron {
            name: name.to_string(),
            borrowed_books_isbn: Vec::new()
        }
    }
}