use tokio::join;
use tokio::time::{Duration, sleep};

async fn check_website(url: &str) -> Result<bool, String> {
    // Simulating a web request
    println!("Checking {}...", url);
    sleep(Duration::from_millis(500)).await;
    Ok(true)
}

#[tokio::main]
async fn main() {
    let sites = vec![
        "https://rust-lang.org",
        "https://crates.io",
        "https://docs.rs",
    ];
    let site1_check = check_website(sites[0]);
    let site2_check = check_website(sites[1]);
    let site3_check = check_website(sites[2]);

    // Wait for all checks to complete concurrently
    let (r1, r2, r3) = join!(site1_check, site2_check, site3_check);

    // Print results
    for (site, result) in [(sites[0], r1), (sites[1], r2), (sites[2], r3)] {
        match result {
            Ok(true) => println!("{} is up!", site),
            _ => println!("{} might be down", site),
        }
    }
}
