#[allow(dead_code)]

struct NowPlaying<'a> {
    artist: &'a str,
    title: &'a str,
    album: &'a str,
    duration_seconds: u32,
}

impl<'a> NowPlaying<'a> {
    fn full_description(&self) -> String {
        format!("{} by {} (from {})", self.title, self.artist, self.album)
    }
}

fn main() {
    let artist_name = String::from("The Beatles");
    let song_title = String::from("Here Comes the Sun");
    let album_name = String::from("Abbey Road");
    
    let current = NowPlaying {
        artist: &artist_name,
        title: &song_title,
        album: &album_name,
        duration_seconds: 185,
    };
    
    println!("{}", current.full_description());
}
