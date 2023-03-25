use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://apc-cap.ic.gc.ca/datafiles/amateur_delim.zip";
    
    // Make a GET request to the server and retrieve the ETag and Last-Modified headers
    let response = reqwest::get(url).await?;
    let etag = response.headers().get("ETag");
    let last_modified = response.headers().get("Last-Modified");
    
    if let Some(etag_value) = etag {
        println!("The ETag of {} is {}", url, etag_value.to_str()?);
    } else {
        println!("The ETag of {} could not be retrieved", url);
    }
    
    if let Some(last_modified_value) = last_modified {
        println!("The Last-Modified header of {} is {}", url, last_modified_value.to_str()?);
    } else {
        println!("The Last-Modified header of {} could not be retrieved", url);
    }
    
    Ok(())
}
