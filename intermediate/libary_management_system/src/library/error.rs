
#[derive(Debug)]
pub enum LibraryError {
    BookNotFound,
    BookNotAvailable,
    PatronNotFound,
    BookAlreadyBorrowed,
    BookNotBorrowed,
    BookAlreadyExist,
    PatronBookNotBorrowed,
    PatronAlreadyExist
}