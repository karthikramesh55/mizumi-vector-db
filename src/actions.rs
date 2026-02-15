#[derive(Debug, Clone)]
 
pub enum Action
{
    Add,
    Search,
    List,
}

impl Action
{
    /**
    * Converting the string representation of an action into the corresponding `Action` enum variant.
    *
    * This function takes a string slice and attempts to match it against known action keywords.
    * The matching is case-insensitive.
    *
    * # Arguments
    *
    * * `action` - A string slice representing the action command (e.g., "add", "ingest")
    *
    * # Returns
    *
    * Returns `Some(Action::Add)`, for example, if the input matches for "add" or "ingest" (case-insensitive),
    * otherwise returns `None` if no matching action is found.
    *
    * `Some` is a variant of the `Option<T>` enum that indicates a successful match was found,
    * while `None` indicates no matching action was found.
    *
    * # Examples
    *
    * ```
    * assert_eq!(Action::from_action_string_to_action_enum("add"), Some(Action::Add));
    * assert_eq!(Action::from_action_string_to_action_enum("INGEST"), Some(Action::Add));
    * assert_eq!(Action::from_action_string_to_action_enum("unknown"), None);
    * ```
    */
    pub fn from_action_string_to_action_enum(action: &str) -> Option<Self>
    {
        match action.to_lowercase().as_str()
        {
            "add" | "ingest" => Some(Action::Add),
            "search" | "query" => Some(Action::Search),
            "list" | "ls" => Some(Action::List),
            _ => None,
            
        }
    }
}