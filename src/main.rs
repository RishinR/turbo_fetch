use futures::future::join_all;
use std::time::Instant;
use tokio::fs;
use turbo_fetch::{download_file, read_links};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // Create downloads directory if it doesn't exist
    fs::create_dir_all("downloads").await?;

    let links = read_links("links.txt").await?;
    let mut tasks = Vec::new();
    for (file_name, link) in links {
        let dest = format!("downloads/{}", file_name);
        tasks.push(download_file(link, dest));
    }
    // Run all downloads concurrently
    join_all(tasks).await.into_iter().collect::<Result<Vec<_>, _>>()?;

    println!("All downloads completed in {:?}", start.elapsed());
    Ok(())
}

// use turbo_fetch::sequential_file_download;
// fn main() {
//     let start = Instant::now();
//     let file_path = "links.txt";
//     if let Err(e) = sequential_file_download(file_path) {
//         eprintln!("Error: {}", e);
//     }
//     println!("All downloads completed in {:?}", start.elapsed());
// }
