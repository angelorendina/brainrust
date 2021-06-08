mod cli;
mod lang;

fn main() {
    match cli::execute() {
        Ok(_) => {}
        Err(err) => match err {
            cli::CliError::Arguments(msg) => {
                println!("{}", msg);
            }
            cli::CliError::File(e) => {
                println!("{:?}", e);
            }
            cli::CliError::Syntax => {
                println!("Syntax error.");
            }
        },
    }
}
