pub fn ingest_url(url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    if !url.starts_with("http") || !url.starts_with("https")
    {
        /* 
        Note: The "string".into() converts the string into the return type we need (Box<dyn std::error::Error>).
              This makes the valid error propagation work seamlessly.
        */
        return Err(format!("Invalid target URL: '{}'. Must start with http or https", url).into());
    }

    println!("Ingesting the target URL: {}", url);
    Ok(())

}
