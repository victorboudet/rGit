pub mod cli;
pub mod commands;
pub mod utils;

use cli::Commands;

fn main() {
    let cli = cli::parse_args();

    match cli.command {
        Commands::Init => {
            commands::init::run();
        }
        Commands::Add { files } => {
            commands::add::run(files);
        }
        Commands::Commit { message } => {
            commands::commit::run(message);
        }
        Commands::Status => {
            commands::status::run();
        }
        Commands::Log => {
            commands::log::run();
        }
    }
}
