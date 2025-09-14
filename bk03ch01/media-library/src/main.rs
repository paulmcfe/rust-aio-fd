#![allow(dead_code)]

use std::fmt;

// Core trait for anything that can be rated
trait Rateable {

    fn rate(&mut self, score: u8) -> Result<(), String>;
    fn get_rating(&self) -> Option<f32>;
    fn get_review_count(&self) -> u32;
}

// Trait for media that can be played
trait Playable {
    fn play(&self) -> String;
    fn get_duration(&self) -> u32; // seconds
}

// Trait for media that can be searched
trait Searchable {
    fn matches_query(&self, query: &str) -> bool;
    fn get_searchable_text(&self) -> String;
}

#[derive(Debug, Clone)]
struct Movie {
    title: String,
    director: String,
    year: u32,
    duration_minutes: u32,
    ratings: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author:String,
    pages: u32,
    ratings: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    duration_seconds: u32,
    ratings: Vec<u8>,
}

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

impl Rateable for Movie {
    fn rate(&mut self, score: u8) -> Result<(), String> {
        if score < 1 || score > 10 {
            return Err("Movie rating must be between 1 and 10".to_string());
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

impl Rateable for Book {
    fn rate(&mut self, score: u8) -> Result<(), String> {
        if score < 1 || score > 5 {
            return Err("Book rating must be between 1 and 5".to_string());
        }
        // Store as doubled value to normalize with movies
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

impl Playable for Movie {
    fn play(&self) -> String {
        format!("Now playing: \"{},\" directed by {} ({}).", self.title, self.director, self.year)
    }

    fn get_duration(&self) -> u32 {
        // Convert to seconds
        self.duration_minutes * 60
    }
}

impl Playable for Song {
    fn play(&self) -> String {
        format!("Now playing: \"{},\" by {} from the album \"{},\".", self.title, self.artist, self.album)
    }

    fn get_duration(&self) -> u32 {
        self.duration_seconds
    }
}

impl Searchable for Movie {
    fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        let searchable = self.get_searchable_text().to_lowercase();
        searchable.contains(&query)
    }

    fn get_searchable_text(&self) -> String {
        format!("{} {} {}", self.title, self.director, self.year)
    }
}

impl Searchable for Book {
    fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        let searchable = self.get_searchable_text().to_lowercase();
        searchable.contains(&query)
    }

    fn get_searchable_text(&self) -> String {
        format!("{} {}", self.title, self.author)
    }
}

impl Searchable for Song {
    fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        let searchable = self.get_searchable_text().to_lowercase();
        searchable.contains(&query)
    }

    fn get_searchable_text(&self) -> String {
        format!("{} {} {}", self.title, self.artist, self.album)
    }
}

impl fmt::Display for Movie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" ({}) directed by {}", self.title, self.year, self.director)
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" by {} ({} pages)", self.title, self.author, self.pages)
    }
}
    
impl fmt::Display for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" by {} from \"{}\"", self.title, self.artist, self.album)
    }
}

struct MediaLibrary {
    name: String,
    movies: Vec<Movie>,
    books: Vec<Book>,
    songs: Vec<Song>,
}

impl MediaLibrary {
    fn new(name: String) -> Self {
        MediaLibrary {
            name,
            movies: Vec::new(),
            books: Vec::new(),
            songs: Vec::new(),
        }
    }
    
    fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }
    
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn search_movies(&self, query: &str) -> Vec<&Movie> {
        self.movies.iter()
            .filter(|movie| movie.matches_query(query))
            .collect()
    }

    fn search_books(&self, query: &str) -> Vec<&Book> {
        self.books.iter()
            .filter(|book| book.matches_query(query))
            .collect()
    }

    fn search_songs(&self, query: &str) -> Vec<&Song> {
        self.songs.iter()
            .filter(|song| song.matches_query(query))
            .collect()
    }

    fn get_stats(&self) -> String {
        format!("Library \"{}\": {} movies, {} books, {} songs",
            self.name, self.movies.len(), self.books.len(), self.songs.len())
    }
}

fn main() {
    let mut library = MediaLibrary::new("My Mighty Media Library".to_string());

    // Add some movies
    let mut movie1 = Movie::new(
        "Best in Show".to_string(), 
        "Christopher Guest".to_string(),
        2000,
        90,
    );
    movie1.rate(9).unwrap();
    movie1.rate(10).unwrap();
    movie1.rate(8).unwrap();

    let mut movie2 = Movie::new(
        "Must Love Dogs".to_string(), 
        "Gary David Goldberg".to_string(),
        2005,
        98,
    );
    movie2.rate(9).unwrap();
    movie2.rate(9).unwrap();
    movie2.rate(8).unwrap();
    movie2.rate(10).unwrap();

    library.add_movie(movie1);
    library.add_movie(movie2);

    // Add some books
    let mut book1 = Book::new(
        "The Curious Incident of the Dog in the Night Time".to_string(),
        "Mark Haddon".to_string(),
        274,
    );
    book1.rate(4).unwrap();
    book1.rate(3).unwrap();
    book1.rate(2).unwrap();
    book1.rate(4).unwrap();
    book1.rate(5).unwrap();

    let mut book2 = Book::new(
        "Timbuktu".to_string(),
        "Paul Auster".to_string(),
        181,
    );
    book2.rate(4).unwrap();
    book2.rate(5).unwrap();
    book2.rate(4).unwrap();
    book2.rate(5).unwrap();

    let mut book3 = Book::new(
        "A Dog's Purpose".to_string(),
        "W. Bruce Cameron".to_string(),
        336,
    );
    book3.rate(3).unwrap();
    book3.rate(4).unwrap();
    book3.rate(3).unwrap();
    book3.rate(4).unwrap();

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // Add some songs
    let mut song1 = Song::new(
        "Who Let the Dogs Out".to_string(),
        "Baha Men".to_string(),
        "Who Let the Dogs Out".to_string(),
        212,
    );
    song1.rate(8).unwrap();
    song1.rate(7).unwrap();
    song1.rate(7).unwrap();

    let mut song2 = Song::new(
        "Martha My Dear".to_string(),
        "The Beatles".to_string(),
        "The Beatles".to_string(),
        148,
    );
    song2.rate(8).unwrap();
    song2.rate(7).unwrap();
    song2.rate(7).unwrap();

    library.add_song(song1);
    library.add_song(song2);

    // Show library stats
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
