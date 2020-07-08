mod worker;
mod options;
mod compile;
mod transform;
mod printable_error;

use crate::worker::process;
use crate::options::get_options;


const HELP: &str = r#"
γsv: standardize and process CSV files

Syntax:

  cat original.csv | ysv configuration.yaml > result.csv

Error messages are printed to stderr.

To get errors in JSON format for integration with other tools:

  cat original.csv | ysv configuration.yaml > result.csv
"#;


// Did the user request the built-in help message?
fn is_help(args: &Vec<String>) -> bool {
    args.get(1).map(
        |argument_value| argument_value.as_str() == "--help"
    ).unwrap_or(
        false,
    )
}


pub fn run(args: Vec<String>) -> Result<(), String> {
    if is_help(&args) {
        eprintln!("{}", HELP);
        Ok(())
    } else {
        let options = get_options(args)?;
        process(options)
    }
}
