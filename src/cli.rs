use compose_parser::Content;

pub enum Cli {
    Help,
    Author,
    File(String),
    Unknown(String),
    Empty,
}

impl Cli {
    pub fn parse(args: &Vec<String>) -> Self {
        if args.len() < 2 {
            return Cli::Empty;
        }

        let command = args.get(1).expect("No command provided");

        match command.as_str() {
            "-h" | "--help" => Cli::Help,
            "--author" => Cli::Author,
            "--file" => {
                if args.len() < 3 {
                    return Cli::Unknown("No file provided".to_string());
                }
                let file = args.get(2).expect("No file provided");
                Cli::File(file.to_string())
            }
            _ => Cli::Unknown(command.to_string()),
        }
    }

    pub(crate) fn parse_command_help() {
        println!("Usage: --file <path>");
        println!("Usage: --author");
    }

    pub(crate) fn parse_command_file(args: &str) {
        println!("Uses file command");

        let file_content = std::fs::read_to_string(args).expect("Unable to read file");

        let result = compose_parser::parse_composable_content(&file_content);

        match result {
            Ok(content) => {
                for item in content {
                    match item {
                        Content::FunctionName(name) => {
                            println!("Function name: {}", name);
                        }
                        Content::Text(name) => {
                            println!("Text: {}", name);
                        }
                        Content::Image(name) => {
                            println!("Image: {}", name);
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error: {:?}", error)
            }
        }
    }

    pub(crate) fn parse_command_author() {
        println!("Author Dzhos Oleksii me@programistich.com");
    }

    pub(crate) fn parse_command_unknown(args: String) {
        println!("Unknown command: {}", args);
        Cli::parse_command_help();
    }

    pub(crate) fn parse_command_empty() {
        println!("No command provided");
        Cli::parse_command_help();
    }
}
