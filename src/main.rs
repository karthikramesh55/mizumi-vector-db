use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let action: String;

    if args.len() < 2
    {
        println!("Usage: cargo run <action>");
        return;
    }

    action = args[1].clone();

    println!("The action you had requested is: {}", action);
}
