use clap::{arg, command, Arg, ArgAction, Command, Subcommand};
use dotenvy::dotenv;
use log::{error, info};

fn main() {
    dotenv().ok();
    env_logger::init();

    // cargo run --bin cd-rs -- -BZ target_folder_val foo_val
    let arg_matches = command!()
        .arg(arg!(<target_folder>).required(false))
        .arg(
            Arg::new("follow_symlink")
                .short('L')
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("physical_dir")
                .short('P')
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exit_nonzero")
                .short('e')
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    info!("full matches: {:?}", arg_matches);

    let target_folder = arg_matches.get_one::<String>("target_folder");

    info!("target_folder matches: {:?}", target_folder);

    info!(
        "follow_symlink matches: {:?}",
        arg_matches.get_one::<bool>("follow_symlink")
    );
    info!(
        "physical_dir matches: {:?}",
        arg_matches.get_one::<bool>("physical_dir")
    );
    info!(
        "exit_nonzero matches: {:?}",
        arg_matches.get_one::<bool>("exit_nonzero")
    );

    if let Some(target_folder) = target_folder {
        let dir_change_result = std::env::set_current_dir(target_folder);
        info!("dir_change_result: {:?}", dir_change_result);

        if let Err(err) = dir_change_result {
            error!("An error occured: {:?}", err);
        }
    }
}
