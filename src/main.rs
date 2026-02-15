use std::env;
use std::process;

mod ingest;
mod models;
mod actions;

use actions::Action;

/*
Note: Result<T, E> facilitates scenario handling (success, failure), where:
- T describes any successful handling
  For example: () is a unit type for describing the aspect of "the flow has ended okay" while returning nullity
- E describes any error handling
  For example: Box<dyn std::error::Error> describes the accommodation of any encountered error

Note: The blocking aspect dedicates system time and effort on a single task
      The "async" keyword allows a function to yield control of a task while waiting for I/O
      This delivers an efficient use of system time and effort

Note: The tokio::main macro transforms the asynchronous main function into a synchronous one 
         for initalizing the Tokio runtime scheduler and executes the asynchronous source code
      Since we have transformed the main function into an asynchronous function,
         any function call inside that whichever performs I/O should be transformed into an asychronous function as well. 
*/

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        eprintln!("Please follow the usage format: cargo run <action> <<target_url>>");
        process::exit(1);
    }

    let action_string = &args[1];
    let received_action = match Action::from_action_string_to_action_enum(action_string)
    {
        Some(successfully_validated_action) => successfully_validated_action,
        None => {
            eprintln!("An invalid action has been received: {}", action_string);
            process::exit(1);
        }
    };

    match received_action
    {
        Action::Add => {
            if args.len() < 3
            {
                eprintln!("Usage: cargo run add <target_url>");
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
            If it does so, the "containing" flow stops, and returns that error immediately
            In this case: the main flow would stop, and returns that error immediately.

            */
            let target = &args[2];
            let bookmark = models::Bookmark::new(target);

            // Note: Since ingest_url has been transformed in to an asynchronous function, .await is used during the call
            ingest::ingest_url(&bookmark.url).await?;
            println!("A bookmark has been built: {:?}", bookmark); // Note: {:?} facilitates the displaying of any structure
        }

        Action::Search => {
            println!("Searching functionality will be implemented in the coming days...");
        }

        Action::List => {
            println!("Listing functionality will be implemented in the coming days...");
        }
    }

    Ok(()) // Note: The Ok() describes the successful completion of the main flow, and when () is passed as an argument onto Ok(), that describes the unit type for returning of nullity
}
