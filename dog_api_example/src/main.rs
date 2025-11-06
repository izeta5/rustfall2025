use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

#[derive(Debug)]
enum DownloadError {
    FetchError(String),
    FileError(String),
    WriteError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => {
            let error_details = format!("Request failed: {}", e);
            ApiResult::NetworkError(error_details)
        },
    }
}

fn download_image(url: &str, filename: &str) -> Result<(), DownloadError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| DownloadError::FetchError(format!("Failed to fetch image: {}", e)))?;

    let mut bytes = Vec::new();
    response.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| DownloadError::FetchError(format!("Failed to read image data: {}", e)))?;

    let mut file = File::create(filename)
        .map_err(|e| DownloadError::FileError(format!("Failed to create file {}: {}", filename, e)))?;

    file.write_all(&bytes)
        .map_err(|e| DownloadError::WriteError(format!("Failed to write to file {}: {}", filename, e)))?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    std::fs::create_dir_all("dog_images")
        .expect("Failed to create dog_images directory");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);
                println!("📊 Status: {}", dog_image.status);

                let extenstion = dog_image.message
                    .split('.')
                    .last()
                    .unwrap_or("jpg");

                let filename = format!("dog_images/dog_{}.{}", i, extenstion);

                print!("⬇️ Downloading to {}... ", filename);
                match download_image(&dog_image.message, &filename) {
                    Ok(()) => println!("✅ Downloaded successfully!"),
                    Err(e) => println!("❌ Download failed: {:?}", e),
                }
            },
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }
    println!("🎉 All done! Check the 'dog_images' folder for your downloaded images.");
    Ok(())
}