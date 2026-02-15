pub fn ingest_url(url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    if !url.starts_with("http") || !url.starts_with("https")
    {
        return Err(format!("Invalid target URL: '{}'. Must start with http or https", url).into());
    }

    println!("Ingesting the target URL: {}", url);
    Ok(())

}
