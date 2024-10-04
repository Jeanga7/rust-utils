pub mod library {
    pub mod writers {
        use crate::library::books::Book;

        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }

    pub mod books {
        #[derive(Debug)]
        pub struct Book {
            pub title: String,
            pub year: u64,
        }
    }
}

pub fn order_books(writer: &mut library::writers::Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}

