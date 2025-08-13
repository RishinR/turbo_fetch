use reqwest;
use tokio::fs;
use std::fs::File;
use std::io::Write;
use tokio::io::AsyncWriteExt;

// Read the links from the file asynchronously
pub async fn read_links(
    file_path: &str,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path).await?;
    let links: Vec<(String, String)> = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, ' ');
            match (parts.next(), parts.next()) {
                (Some(file_name), Some(link)) if !file_name.is_empty() && !link.is_empty() => {
                    Some((file_name.to_string(), link.to_string()))
                }
                _ => None,
            }
        })
        .collect();
    Ok(links)
}

// Download file from the link asynchronously
pub async fn download_file(
    link: String,
    destination: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(&link).await?;
    let content = response.bytes().await?;
    let mut file = fs::File::create(&destination).await?;
    file.write_all(&content).await?;
    Ok(())
}

pub fn sequential_file_download(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Read links from file
    let content = std::fs::read_to_string(file_path)?;
    for line in content.lines() {
        let mut parts = line.splitn(2, ' ');
        let file_name = parts.next().unwrap_or("");
        let link = parts.next().unwrap_or("");
        if file_name.is_empty() || link.is_empty() {
            continue;
        }
        let resp = reqwest::blocking::get(link)?;
        let bytes = resp.bytes()?;
        let mut file = File::create(format!("downloads/{}", file_name))?;
        file.write_all(&bytes)?;
    }
    Ok(())
}
