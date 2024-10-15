pub struct Writer {
    first_name: String,
    last_name: String,
    books: Vec<Book>,
}
pub struct Book {
    title: String,
    year: u64,
}
pub fn order_books(writer: &mut Writer) {
    writer.books.sort()
}
