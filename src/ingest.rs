use reqwest;
use crate::models::{HtmlContent, Cleanable, Bookmark};

pub async fn ingest_url(url: &str) -> Result<Bookmark, Box<dyn std::error::Error>>
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

    let raw_html = HtmlContent{ url: url.to_string(), raw_text: response_text};
    println!("Cleaning the text content...");

    let cleaned_content = raw_html.clean()?;

    println!("----------------------------------------------------------");
    println!("Title:       {}", cleaned_content.title);
    println!("Content:     {} bytes", cleaned_content.content.len());
    println!("Snippet:     {:.160}...", cleaned_content.content);
    println!("----------------------------------------------------------");

    Ok(Bookmark
        {
            url: url.to_string(),
            title: cleaned_content.title,
            bookmark_content: cleaned_content.content
        })  // Note: The Ok() describes the successful completion of this flow, and when () is passed as an argument onto Ok(), that describes the unit type for the returning of nullity onto the calling point.
}
