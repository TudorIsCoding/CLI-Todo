use std::env;
use std::error::Error;
use std::process;
use todo_list::run;
use todo_list::Config;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)?;

    if let Err(e) = run(config) {
        println!("{e}");
        process::exit(1);
    }
    Ok(())
}
