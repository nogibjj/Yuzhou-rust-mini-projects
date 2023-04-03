use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    title: String,
    author: String,
    genre: String,
    year: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the JSON file containing the list of books
    let mut file = File::open("books.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON data into a vector of books
    let books: Vec<Book> = serde_json::from_str(&contents)?;

    // Prompt the user to enter a search query
    println!("Enter a search query (title, author, or genre):");
    let mut search_query = String::new();
    std::io::stdin().read_line(&mut search_query)?;

    // Filter the vector of books based on the search query
    let filtered_books = books.into_iter()
        .filter(|book| {
            book.title.contains(&search_query.trim())
            || book.author.contains(&search_query.trim())
            || book.genre.contains(&search_query.trim())
        })
        .collect::<Vec<_>>();

    if filtered_books.is_empty() {
        println!("No books found!");
        return Ok(());
    } else {
        println!("Found {} books:", filtered_books.len());
    }
    // Output the filtered list of books to the console
    for book in filtered_books {
        println!("{} by {} ({}, {})", book.title, book.author, book.genre, book.year);
    }

    Ok(())
}
