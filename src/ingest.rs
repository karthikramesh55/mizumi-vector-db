use reqwest::Url;
use readability::extractor;

pub async fn ingest_url(url: &str) -> Result<(), Box<dyn std::error::Error>>
{
    if !url.starts_with("http") || !url.starts_with("https")
    {
        /* 
        Note: The "string".into() converts the string into the return type we need (Box<dyn std::error::Error>).
              This makes the valid error propagation work seamlessly.
        */
        return Err(format!("Invalid target URL: '{}'. Must start with http or https accordingly", url).into());
    }

    println!("Accessing the URL: {}", url);

    /*
    Note: We are fetching the raw HTML using the reqwest crate utility in an asynchronous manner
          The .await enables the yielding of control of the task, until the network transaction finishes
          Meanwhile, the control moves to another task and comes back to this task after completion of the network transaction
    */
    let response_text = reqwest::get(url).await?.text().await?;

    println!("Fetched {} bytes from the URL. Extracting the text content...", response_text.len());

    /*
    Note: We are parsing the raw HTML using the readability::extractor crate utility to get the primary text content
             In this regard, the extractor requires a stream wrapper (i.e. cursor objectual body) + URL objectual body to resolve relative links
          The cursor that is wrapping the response buffer implements Read + Seek, that is used to read + navigate within the in-memory data.
             We make the cursor objectual body mutable so its internal position can be updated during read + seek operations.
    */
    let mut cursor = std::io::Cursor::new(response_text);
    let url_objectual_body = Url::parse(url)?;

    let resultant_product = extractor::extract(&mut cursor, &url_objectual_body)?;

    println!("---------------------------------------------");
    println!("Title:       {}", resultant_product.title);
    println!("Content:     {} bytes", resultant_product.text.len());
    println!("Snippet:     {:.300}...", resultant_product.text);

    Ok(())  // Note: The Ok() describes the successful completion of this flow, and when () is passed as an argument onto Ok(), that describes the unit type for returning of nullity
}
