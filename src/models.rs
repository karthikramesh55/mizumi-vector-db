use readability::extractor;
use std::io::Cursor;
use reqwest::Url;

#[derive(Debug, Clone)]
/*
Note: Debug allows us to display the Bookmark structure using {:?}
      Clone allows us to make replicas of the Bookmark structure
*/
pub struct Bookmark
{
    pub url: String,
    pub title: String,
    pub bookmark_content: String,
}

impl Bookmark
{
    // Note: A 1-parameter infusing constructor that accepts a URL and characterizes the bookmark during inception
    pub fn new(url: &str) -> Self
    {
        Self
        {
            url: url.to_string(),             // Note: The accepted URL value is assigned to the structure
            title: String::new(),             // Note: Empty for now, this will be filled later
            bookmark_content: String::new(),  // Note: Empty for now, this will be filled later
        }
    }
}

pub struct HtmlContent
{
    pub url: String,
    pub raw_text: String,
} // Note: Defining the HtmlContent structure for wrapping the raw HTML content that will have to be cleaned.

#[derive(Debug)]
pub struct CleanedData
{
    pub title: String,
    pub content: String,
} // Note: Defining the CleanedData structure in order to describe as to how the cleaned data should be characterized.

pub trait Cleanable
{
    fn clean(&self) -> Result<CleanedData, Box<dyn std::error::Error>>;
} // Note: Defining the Cleanable trait (i.e. an interface for shared behavior) that establishes a contract where the raw (HTML, PDF, Text file, Markdown snippet) agrees to be transformed into CleanedData.

impl Cleanable for HtmlContent
{
    fn clean(&self) -> Result<CleanedData, Box<dyn std::error::Error>>
    {
        /*
        Note: We are parsing the raw HTML using the readability::extractor crate utility to extract the title and the text content
                In this regard, the extractor requires a stream wrapper (i.e. cursor objectual body) + URL objectual body to resolve relative links
              The cursor that is wrapping the response buffer implements Read + Seek trait, that is used to read + navigate the in-memory data.
                We make the cursor objectual body mutable so that its internal position can be updated during the read + seek operation.
        */
        let mut cursor = Cursor::new(&self.raw_text);
        let url_objectual_body = Url::parse(&self.url)?;

        let resultant_product = extractor::extract(&mut cursor, &url_objectual_body)?; // Note: The extractor gives us the title and the text content

        Ok(CleanedData
            {
                title: resultant_product.title,
                content: resultant_product.text
            })
    }
} // Note: Implementing the trait (i.e. behavior) for the HtmlContent structure for cleaning the raw HTML data fetched from the target URL.
