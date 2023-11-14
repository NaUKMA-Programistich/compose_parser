use crate::cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::parse(&args);

    match cli {
        Cli::Help => Cli::parse_command_help(),
        Cli::Author => Cli::parse_command_author(),
        Cli::File(file) => Cli::parse_command_file(file.as_str()),
        Cli::Unknown(command) => Cli::parse_command_unknown(command),
        Cli::Empty => Cli::parse_command_empty(),
    }

    Ok(())
}
