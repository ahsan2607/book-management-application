// Made by Ahsan
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan notes
#[contracttype]
#[derive(Clone, Debug)]
pub struct Book {
    id: u64,
    title: String,
    content: String,
    author: String,
    pages: u64,
}

// Storage key untuk data notes
const BOOK_DATA: Symbol = symbol_short!("BOOK_DATA");

#[contract]
pub struct BooksContract;

#[contractimpl]
impl BooksContract {
    // Fungsi untuk mendapatkan semua notes
    pub fn get_books(env: Env) -> Vec<Book> {
        // 1. ambil data notes dari storage
        return env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat note baru
    pub fn create_book(env: Env, title: String, content: String, author: String, pages: u64) -> String {
        // 1. ambil data notes dari storage
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object note baru
        let book = Book {
            id: env.prng().gen::<u64>(),
            title: title,
            content: content,
            author: author,
            pages: pages,
        };
        
        // 3. tambahkan note baru ke notes lama
        books.push_back(book);
        
        // 4. simpan notes ke storage
        env.storage().instance().set(&BOOK_DATA, &books);
        
        return String::from_str(&env, "Book berhasil ditambahkan");
    }

    // Fungsi untuk mengedit book berdasarkan id
    pub fn update_book(env: Env, id: u64, title: String, content: String, author: String, pages: u64) -> String {
        // 1. ambil data notes dari storage 
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index book yang akan diupdate
        for i in 0..books.len() {
            if books.get(i).unwrap().id == id {
                let updated_book = Book {
                    id: id,
                    title: title,
                    content: content,
                    author: author,
                    pages: pages,
                };

                books.set(i, updated_book);

                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Berhasil update book");
            }
        }

        return String::from_str(&env, "Book tidak ditemukan")
    }

    // Fungsi untuk menghapus notes berdasarkan id
    pub fn delete_book(env: Env, id: u64) -> String {
        // 1. ambil data notes dari storage 
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index note yang akan dihapus menggunakan perulangan
        for i in 0..books.len() {
            if books.get(i).unwrap().id == id {
                books.remove(i);

                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Berhasil hapus book");
            }
        }

        return String::from_str(&env, "Book tidak ditemukan")
    }
}

mod test;