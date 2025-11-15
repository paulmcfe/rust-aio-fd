use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    let items = 1000;
    let pb = ProgressBar::new(items);

    // Customize the appearance
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    for _ in 0..items {
        thread::sleep(Duration::from_millis(10));
        pb.inc(1);
    }

    pb.finish_with_message("Complete!");
}
