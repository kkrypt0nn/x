use std::{
    env,
    fs::File,
    io::{self, Read},
    process::exit,
};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 || !args[1].ends_with(".bf") {
        println!("Usage: ./bfrs <filename>.bf");
        exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    bfrs::Machine::new().evaluate(buffer);

    Ok(())
}
