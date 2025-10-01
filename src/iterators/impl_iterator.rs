//! src/iterators/impl_iterator.rs
#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }

    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }
            None => {
                println!("Out of books in the library!");
                None
            }
        }
    }
}

pub fn index(show: bool) {
    if show {
        println!("Implementing iterator for your own types");

        let mut my_library = Library::new("Calgary");

        my_library.add_book("The Doom of the Darksword");
        my_library.add_book("Demina - die Geschichte einer Jugend");
        my_library.add_book("Tesla - Wizard at War");
        my_library.add_book("Anna Karenina");

        my_library.name = "Learning Rust".to_string();

        for item in my_library.get_books() {
            println!("{item}");
        }
    }
}
