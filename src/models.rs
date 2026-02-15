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