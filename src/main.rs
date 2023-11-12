use crate::cli::CLI;

mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = CLI::parse(&args);

    match cli {
        CLI::Help => CLI::parse_command_help(),
        CLI::Author => CLI::parse_command_author(),
        CLI::File(file) => CLI::parse_command_file(file),
        CLI::Unknown(command) => CLI::parse_command_unknown(command),
        CLI::Empty => CLI::parse_command_empty(),
    }
}