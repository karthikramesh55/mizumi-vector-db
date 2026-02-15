use std::env;
use std::process;

mod ingest;
mod models;
mod actions;

use actions::Action;

/*
Note: Result<T, E> facilitates scenario handling (success, failure), where:
- T describes any successful handling
  For example: () is a unit type for describing an "okay" signaling while returning nullity
- E describes any error handling
  For example: Box<dyn std::error::Error> describes the accommodation of any encountered error
*/
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("Please follow the usage format: cargo run <action> <target>");
        process::exit(1);
    }

    let action_string = &args[1];
    let received_action = match actions::Action::from_action_string_to_action_enum(action_string)
    {
        Some(successfully_validated_action) => successfully_validated_action,
        None => {
            eprintln!("Invalid action: {}", action_string);
            process::exit(1);
        }
    };

    match received_action
    {
        Action::Add => {
            if args.len() < 3
            {
                eprintln!("Usage: cargo run add <url>");
                process::exit(1);
            }
            /*
            Syntactic sugar equivalency:
            ---------------------------
            match do_something()
            {
                Ok(value) => value,
                Err(e) => return Err(e),
            }

            |
            |
            |

            let value = do_something()?;

            The '?' operator checks as to whether do_something() returns Err
            If it does so, the "containing" flow stops, and returns that error automatically
            In this case: the main flow would stop, and return that error automatically.

            */
            let target = &args[2];
            let bookmark = models::Bookmark::new(target);
            ingest::ingest_url(&bookmark.url)?;
            println!("A bookmark has been built: {:?}", bookmark); // Note: {:?} facilitates the displaying of any structure
        }

        Action::Search => {
            println!("Searching functionality will be implemented in the coming days...");
        }

        Action::List => {
            println!("Listing functionality will be implemented in the coming days...");
        }
    }

    Ok(()) // Note: The Ok() describes the successful completion of the matching aspect, when () is passed as an argument, that describes the unit type for nullity
}
