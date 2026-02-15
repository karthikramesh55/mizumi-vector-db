pub fn ingest_url(url: &str) -> Result<(), Box<dync std::error::Error>>
{
    if !url.starts_with("http")
    {
        return Err(format!("Invalid target URL: '{}'. Must start with http or https", url).into());
    }

    println!("Ingesting the target URL: {}", url);
    Ok(());

}
