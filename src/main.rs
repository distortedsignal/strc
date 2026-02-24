use std::env;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <prog> <args>", args[0]);
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not enough arguments"
            ).into());
    }

    
    println!("Hello, world!");
    Ok(())
}
