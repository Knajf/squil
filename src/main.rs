use std::io::Write;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    loop {
        let mut input = String::new();
        print!("squil> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input)?;
        match input.trim() {
            ".exit" => break,
            ".help" => println!("I need help too :("),
            _ => println!("Unrecognized command '{}'.", input.trim()),
        }
        // println!("{}", input);
    }

    Ok(())
}


