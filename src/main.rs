use std::env;
use std::process;

mod ingest;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3
    {
        eprintln!("Follow the usage format: cargo run <action> <target>");
        process::exit(1);
    }

    let action = &args[1];
    let target = &args[2];

    println!("The action and target URL that you had requested is: cargo run {} {}", action, target);

    match action.as_str()
    {
        "add" | "ingest" => {
            ingest::ingest_url(target)?;
            println!("Successfully processed the target URL: {}", target);
        }
        _ => {
            eprintln!("Unknown action: {}", action);
            process::exit(1);
        }
    }

    Ok(())
}
