#![allow(dead_code)]

// Need this to implement Display
use std::fmt;

// Core trait for any media that can be rated
trait Rateable {
    // Rate the media item
    // This method makes changes to the item's `ratings` vector,
    // so the `self` reference has to be mutable`
    fn rate(&mut self, score: u8) -> Result<(), String>;

    // Get the item's rating
    fn get_rating(&self) -> Option<f32>;

    // Get the number of reviews for the item
    fn get_review_count(&self) -> u32;
}

// Trait for media that can be played
trait Playable {

    // "Play" the media item (but, really, just return a string)
    fn play(&self) -> String;

    // Get the duration of the item
    fn get_duration(&self) -> u32; // seconds
}

// Trait for media that can be searched
trait Searchable {

    // Returns `true` if the `query` string slice is found in the item's data
    fn matches_query(&self, query: &str) -> bool;

    // Returns the item data as a String that can be searched
    fn get_searchable_text(&self) -> String;
}

// Data structure for a movie
// Note that the movie's ratings are stored in the `ratings` vector
#[derive(Debug)]
struct Movie {
    title: String,
    director: String,
    year: u32,
    duration_minutes: u32,
    ratings: Vec<u8>,
}

// Data structure for a book
#[derive(Debug)]
struct Book {
    title: String,
    author:String,
    pages: u32,
    ratings: Vec<u8>,
}

// Data structure for a song
#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    album: String,
    duration_seconds: u32,
    ratings: Vec<u8>,
}

// Implement the constructor for a Movie item
impl Movie {
    fn new(title: String, director: String, year: u32, duration_minutes: u32) -> Self {
        Movie {
            title,
            director,
            year,
            duration_minutes,
            ratings: Vec::new(),
        }
    }
}

// Implement the constructor for a Book item
impl Book {
    fn new(title: String, author: String, pages: u32) -> Self {
        Book {
            title,
            author,
            pages,
            ratings: Vec::new(),
        }
    }
}

// Implement the constructor for a Song item
impl Song {
    fn new(title: String, artist: String, album: String, duration_seconds: u32) -> Self {
        Song {
            title,
            artist,
            album,
            duration_seconds,
            ratings: Vec::new(),
        }
    }
}

// Implement the Rateable trait for a Movie item
impl Rateable for Movie {

    // Rate this item
    fn rate(&mut self, score: u8) -> Result<(), String> {
        // Is the rating within bounds?
        if score < 1 || score > 10 {
            // If not, return a Err string
            return Err("Movie rating must be between 1 and 10".to_string());
        }
        // Otherwise, push the score to the `ratings` vector and return () in Ok()
        self.ratings.push(score);
        Ok(())
    }

    // Return this item's average rating
    fn get_rating(&self) -> Option<f32> {
        // Do we have any ratings yet?
        if self.ratings.is_empty() {
            // If not, return None
            None
        } else {
            // Otherwise, sum the ratings
            let sum: u32 = self.ratings.iter().map(|&r| r as u32).sum();
            // Return the average in the Some
            Some(sum as f32 / self.ratings.len() as f32)
        }
    }

    // Return the number of reviews
    fn get_review_count(&self) -> u32 {
        self.ratings.len() as u32
    }
}

// Implement the Rateable trait for a Book item
impl Rateable for Book {
    fn rate(&mut self, score: u8) -> Result<(), String> {
        // Is the rating within bounds (note: 1 to 5 here)?
        if score < 1 || score > 5 {
            return Err("Book rating must be between 1 and 5".to_string());
        }
        // Store as doubled value to normalize with movies and songs
        self.ratings.push(score * 2);
        Ok(())
    }

    fn get_rating(&self) -> Option<f32> {
        if self.ratings.is_empty() {
            None
        } else {
            let sum: u32 = self.ratings.iter().map(|&r| r as u32).sum();
            // Return the rating using the 5-point scale
            Some((sum as f32 / self.ratings.len() as f32) / 2.0)
        }
    }

    fn get_review_count(&self) -> u32 {
        self.ratings.len() as u32
    }
}

// Implement the Rateable trait for a Song item
impl Rateable for Song {
    fn rate(&mut self, score: u8) -> Result<(), String> {
        if score < 1 || score > 10 {
            return Err("Song rating must be between 1 and 10".to_string());
        }
        self.ratings.push(score);
        Ok(())
    }

    fn get_rating(&self) -> Option<f32> {
        if self.ratings.is_empty() {
            None
        } else {
            let sum: u32 = self.ratings.iter().map(|&r| r as u32).sum();
            Some(sum as f32 / self.ratings.len() as f32)
        }
    }

    fn get_review_count(&self) -> u32 {
        self.ratings.len() as u32
    }
}

// Implement the Playable trait for a Movie item
impl Playable for Movie {
    // Return a String
    fn play(&self) -> String {
        format!("Now playing: \"{},\" directed by {} ({}).", self.title, self.director, self.year)
    }
    // Return the movie's duration, in seconds (this method is not used in this project)
    fn get_duration(&self) -> u32 {
        // Convert to seconds
        self.duration_minutes * 60
    }
}

// Implement the Playable trait for a Song item
impl Playable for Song {

    // Return a String
    fn play(&self) -> String {
        format!("Now playing: \"{},\" by {} from the album \"{},\".", self.title, self.artist, self.album)
    }

    // Return the song's duration (this method is not used in this project)
    fn get_duration(&self) -> u32 {
        self.duration_seconds
    }
}

// Implement the Searchable trait for a Movie item
impl Searchable for Movie {

    // Returns `true` if the `query` string slice is found in the item
    fn matches_query(&self, query: &str) -> bool {

        // Convert the query to lowercase
        let query = query.to_lowercase();

        // Get the item text to be searched and convert it to lowercase
        let searchable = self.get_searchable_text().to_lowercase();

        // Does the searchable text contain `query`?
        searchable.contains(&query)
    }

    // Return a String with the title, director, and year for searching
    fn get_searchable_text(&self) -> String {
        format!("{} {} {}", self.title, self.director, self.year)
    }
}

// Implement the Searchable trait for a Book item
impl Searchable for Book {

    // Returns `true` if the `query` string slice is found in the item
    fn matches_query(&self, query: &str) -> bool {

        // Convert the query to lowercase
        let query = query.to_lowercase();

        // Get the item text to be searched and convert it to lowercase
        let searchable = self.get_searchable_text().to_lowercase();

        // Does the searchable text contain `query`?
        searchable.contains(&query)
    }

    // Return a String with the title and author for searching
    fn get_searchable_text(&self) -> String {
        format!("{} {}", self.title, self.author)
    }
}

// Implement the Searchable trait for a Song item
impl Searchable for Song {

    // Returns `true` if the `query` string slice is found in the item
    fn matches_query(&self, query: &str) -> bool {

        // Convert the query to lowercase
        let query = query.to_lowercase();

        // Get the item text to be searched and convert it to lowercase
        let searchable = self.get_searchable_text().to_lowercase();

        // Does the searchable text contain `query`?
        searchable.contains(&query)
    }

    // Return a String with the title, artist, and album for searching
    fn get_searchable_text(&self) -> String {
        format!("{} {} {}", self.title, self.artist, self.album)
    }
}

// Implement the Display trait for Movie
impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" ({}) directed by {}", self.title, self.year, self.director)
    }
}

// Implement the Display trait for Book
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" by {} ({} pages)", self.title, self.author, self.pages)
    }
}

// Implement the Display trait for Song
impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" by {} from \"{}\"", self.title, self.artist, self.album)
    }
}

// Data structure to hold the media library metadata
struct MediaLibrary {
    name: String,
    movies: Vec<Movie>,
    books: Vec<Book>,
    songs: Vec<Song>,
}

// Define the methods and helper functions for MediaLibrary
impl MediaLibrary {

    // Constructor
    fn new(name: String) -> Self {
        MediaLibrary {
            name,
            movies: Vec::new(),
            books: Vec::new(),
            songs: Vec::new(),
        }
    }
    
    // Add a movie to the library
    fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }
    
    // Add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // Add a song to the library
    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    // Search the movies, returning a vector containing the matches
    fn search_movies(&self, query: &str) -> Vec<&Movie> {
        self.movies.iter()
            .filter(|movie| movie.matches_query(query))
            .collect()
    }

    // Search the books, returning a vector containing the matches
    fn search_books(&self, query: &str) -> Vec<&Book> {
        self.books.iter()
            .filter(|book| book.matches_query(query))
            .collect()
    }

    // Search the songs, returning a vector containing the matches
    fn search_songs(&self, query: &str) -> Vec<&Song> {
        self.songs.iter()
            .filter(|song| song.matches_query(query))
            .collect()
    }

    // Display some basic stats about the media library's contents
    fn get_stats(&self) -> String {
        format!("Library \"{}\": {} movies, {} books, {} songs",
            self.name, self.movies.len(), self.books.len(), self.songs.len())
    }
}

// Main function
fn main() {

    // Create the media library
    let mut library = MediaLibrary::new("My Mighty Media Library".to_string());

    // Create a movie instance
    let mut movie1 = Movie::new(
        "Best in Show".to_string(), 
        "Christopher Guest".to_string(),
        2000,
        90,
    );

    // Rate the movie
    let ratings = [9, 10, 8];
    for rating in ratings {
        if let Err(e) = movie1.rate(rating) {
            println!("Warning: {}", e);
        }
    }

    // Create a movie instance
    let mut movie2 = Movie::new(
        "Must Love Dogs".to_string(), 
        "Gary David Goldberg".to_string(),
        2005,
        98,
    );
    
    // Rate the movie
    let ratings = [9, 9, 8, 10];
    for rating in ratings {
        if let Err(e) = movie2.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Add the movies to the library
    library.add_movie(movie1);
    library.add_movie(movie2);

    // Create a book instance
    let mut book1 = Book::new(
        "The Curious Incident of the Dog in the Night Time".to_string(),
        "Mark Haddon".to_string(),
        274,
    );

    // Rate the book
    let ratings = [4, 3, 2, 4, 5];
    for rating in ratings {
        if let Err(e) = book1.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Create a book instance
    let mut book2 = Book::new(
        "Timbuktu".to_string(),
        "Paul Auster".to_string(),
        181,
    );

    // Rate the book
    let ratings = [4, 5, 4, 5];
    for rating in ratings {
        if let Err(e) = book2.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Create a book instance
    let mut book3 = Book::new(
        "A Dog's Purpose".to_string(),
        "W. Bruce Cameron".to_string(),
        336,
    );

    // Rate the book

    let ratings = [3, 4, 3, 4];
    for rating in ratings {
        if let Err(e) = book3.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Add the books to the library
    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // Create a song instance
    let mut song1 = Song::new(
        "Who Let the Dogs Out".to_string(),
        "Baha Men".to_string(),
        "Who Let the Dogs Out".to_string(),
        212,
    );

    // Rate the song
    let ratings = [8, 7, 7];
    for rating in ratings {
        if let Err(e) = song1.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Create a song instance
    let mut song2 = Song::new(
        "Martha My Dear".to_string(),
        "The Beatles".to_string(),
        "The Beatles".to_string(),
        148,
    );

    // Rate the song
    let ratings = [8, 7, 7];
    for rating in ratings {
        if let Err(e) = song2.rate(rating) {
            println!("Warning: {}", e);
            println!("");
        }
    }

    // Add the songs to the library
    library.add_song(song1);
    library.add_song(song2);

    // Show the library stats
    println!("{}", library.get_stats());
    println!();

    // Demonstrate searchable functionality
    println!("Searching for 'Dog' in books:");
    for book in library.search_books("Dog") {
        println!("    Found book: {}", book);
        if let Some(rating) = book.get_rating() {
            println!("    Average rating: {:.1}/5 ({} reviews)",
                rating, book.get_review_count());
        }
    }
    println!();

    // Demonstrate playable functionality
    println!("Playing media:");
    for movie in &library.movies {
        println!("    {}", movie.play());
    }
    for song in &library.songs {
        println!("    {}", song.play());
    }
    println!()
}
