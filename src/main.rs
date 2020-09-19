mod config;
mod model;
mod tests;

use config::*;
use model::*;
use std::env;
use std::io::prelude::*;

#[derive(Debug)]
struct Error {
    message: String,
}

mod error_conversion { 
    use super::Error;
    use super::config;
    use super::config::params::ErrorType;
    
    impl std::convert::From<std::io::Error> for Error {
        fn from(error: std::io::Error) -> Self {
            return Error {
                message: format!("{}", error),
            };
        }
    }

    impl std::convert::From<&str> for Error {
        fn from(error: &str) -> Self {
            return Error {
                message: format!("{}", error),
            };
        }
    }

    impl std::convert::From<config::params::ErrorType> for Error {
        fn from(error: config::params::ErrorType) -> Self {
            match error {
                ErrorType::MissingArguments(unset_arguments) => {
                    for unset_argument in unset_arguments {
                        eprintln!("error unset argument {}", unset_argument.long_form);
                    }
                    Error {
                        message: "missing arguments".into(),
                    }
                }
                ErrorType::CannotIdentifyCommand(_) => Error {
                    message: "error cannot identify command".into(),
                },
            }
        }
    }
 }

fn run_app(config: Config, command: Command) -> Result<(), Error> {
    let todo_str = std::fs::read_to_string(config.todo_filename.expect(
        "todo filename is not present in config file and --todo-file parameter is missing",
    ))?;
    let done_str = std::fs::read_to_string(config.done_filename.expect(
        "done filename is not present in config file and --done-file parameter is missing",
    ))?;
    let mut model = Model {
        todo_data: TodoData::parse(&todo_str).expect("error parsing todo file"),
        done_data: TodoData::parse(&done_str).expect("error parsing done file"),
    };
    model.execute(command).map_err(|e| Error {
        message: e.to_owned(),
    })
}

fn read_configuration(mut config: &mut Config) {
    if let Option::Some(dirs) = directories::ProjectDirs::from("", "", "todocommander") {
        let mut path = dirs.config_dir().to_path_buf();
        path.push("todocommander.cfg");
        println!("{:?}", path);
        if let Result::Ok(mut f) = std::fs::File::open(path) {
            let mut file_content = String::new();
            let _result = f.read_to_string(&mut file_content);
            read_configuration_from_filecontent(&file_content, &mut config);
        }
    }
}

fn main() -> Result<(), Error> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("{}", VERSION);
    let mut config = Config::new();
    read_configuration(&mut config);
    println!("{:?}", &config);
    let arguments = parse_arguments(&mut env::args())?;
    config = Config {
        todo_filename: arguments.config.todo_filename.or(config.todo_filename),
        done_filename: arguments.config.done_filename.or(config.done_filename),
    };
    run_app(config, arguments.command)
}
