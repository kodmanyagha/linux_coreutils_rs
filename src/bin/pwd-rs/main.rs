use dotenvy::dotenv;
use log::{info, warn};
use std::env;

use clap::{command, Arg, ArgAction, ArgMatches};

#[derive(Debug)]
struct CliParams {
    pub from_pwd: bool,
    pub physical: bool,
}

impl From<ArgMatches> for CliParams {
    fn from(value: ArgMatches) -> Self {
        let mut result = CliParams {
            from_pwd: *value.get_one::<bool>("from_pwd").unwrap_or(&true),
            physical: *value.get_one::<bool>("physical").unwrap_or(&false),
        };

        if !result.physical {
            result.from_pwd = true;
        }

        result
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let matches = command!()
        .arg(Arg::new("from_pwd").short('L').action(ArgAction::SetTrue))
        .arg(Arg::new("physical").short('P').action(ArgAction::SetTrue))
        .get_matches();

    info!(">>> matches: {:?}", matches);

    let cli_params = CliParams::from(matches);
    info!(">> cli_params: {:?}", cli_params);

    if cli_params.from_pwd {
        info!("from_pwd");

        print_from_pwd();
    } else if cli_params.physical {
        info!("physical");

        print_physical();
    } else {
        warn!("Nothing of these cli params.");
    }
}

fn print_physical() {
    let cwd = env::current_dir();

    match cwd {
        Ok(cwd) => println!("{}", cwd.to_str().unwrap()),
        Err(err) => println!("Error occured: {:?}", err),
    }
}

fn print_from_pwd() {
    print_physical();
}
