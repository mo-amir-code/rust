pub mod book;
pub mod patron;
pub mod error;

use book::Book;
use patron::Patron;
use error::LibraryError;

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
    pub patrons: Vec<Patron>
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
            patrons: Vec::new()
        }
    }

    pub fn add_book(&mut self, title: &str, author: &str, isbn: &str) -> Result<(), LibraryError> {      
        if self.books.iter().any(|x| x.isbn == isbn) {
            return Err(LibraryError::BookAlreadyExist);
        }

        self.books.push(Book::new(title, author, isbn));
        Ok(())
    }

    pub fn add_patron(&mut self, name: &str) -> Result<(), LibraryError> {      
        if self.patrons.iter().any(|p| p.name == name) {
            return Err(LibraryError::PatronAlreadyExist);
        }

        self.patrons.push(Patron::new(name));
        Ok(())
    }

    pub fn borrow_book(&mut self, isbn: &str, patron_name: &str) -> Result<(), LibraryError> {
        let book = self.books.iter_mut().find(|x| x.isbn == isbn).ok_or(LibraryError::BookNotFound)?;

        if !book.available {
            return Err(LibraryError::BookNotAvailable);
        }

        let patron = self.patrons.iter_mut().find(|x| x.name == patron_name).ok_or(LibraryError::PatronNotFound)?;
        patron.borrowed_books_isbn.push(isbn.to_string());

        book.available = false;

        Ok(())
    }

    pub fn return_book(&mut self, isbn: &str, patron_name: &str) -> Result<(), LibraryError> {
        let book = self.books.iter_mut().find(|x| x.isbn == isbn).ok_or(LibraryError::BookNotFound)?;

        if book.available {
            return Err(LibraryError::BookNotBorrowed);
        }

        let patron = self.patrons.iter_mut().find(|x| x.name == patron_name).ok_or(LibraryError::PatronNotFound)?;

        if !patron.borrowed_books_isbn.contains(&isbn.to_string()) {
            return Err(LibraryError::PatronBookNotBorrowed)
        }

        patron.borrowed_books_isbn.retain(|x| x != isbn);
        book.available = true;
        Ok(())
    }

    pub fn book_list(&self){
        for b in &self.books{
            print!("{}\n", b.title);
        }
    }

    pub fn available_book_list(&self){
        for b in &self.books{
            if b.available {
                print!("{}\n", b.title);
            }
        }
    }

    pub fn patrons_borrowed_book_list(&self){
        for b in &self.books{
                if !b.available {
                    print!("{}\n", b.title);
                }
        }
    }

    pub fn patrons_list(&self) {
        for p in &self.patrons {
            print!("{}\n", p.name);
        }
    }

}