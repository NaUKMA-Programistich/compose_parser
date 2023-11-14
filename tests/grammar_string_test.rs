mod tests {
    use pest::Parser;
    use compose_parser::{ComposableFunctionParser, Rule};

    #[test]
    // string_image = { "\"" ~ ASCII_ALPHANUMERIC* ~ ".png\"" }
    fn test_string_image() {
        let correct_image = "\"image.png\"";
        let correct_pair = ComposableFunctionParser::parse(
           Rule::string_image,
           correct_image,
        );

        assert!(correct_pair.is_ok());
        assert_eq!(correct_pair.unwrap().as_str(), correct_image);

        let incorrect_pair_bracket = ComposableFunctionParser::parse(
           Rule::string_image,
           "\"image.png",
        );
        assert!(incorrect_pair_bracket.is_err());

        let incorrect_pair_ext = ComposableFunctionParser::parse(
            Rule::string_image,
            "\"image.pngq\"",
        );
        assert!(incorrect_pair_ext.is_err());
    }
    
    #[test]
    // string = { "\"" ~ (ASCII_ALPHANUMERIC | " ")* ~ "\"" }
    fn test_string() {
        let correct_string = "\"string\"";
        let correct_pair = ComposableFunctionParser::parse(
           Rule::string,
           correct_string,
        );

        assert!(correct_pair.is_ok());
        assert_eq!(correct_pair.unwrap().as_str(), correct_string);

        let incorrect_pair_bracket = ComposableFunctionParser::parse(
           Rule::string,
           "\"string",
        );
        assert!(incorrect_pair_bracket.is_err());
    }

    #[test]
    // identifier = { ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
    fn test_identifier() {
        let correct_identifier = "identifier";
        let correct_pair = ComposableFunctionParser::parse(
           Rule::identifier,
           correct_identifier,
        );
        assert!(correct_pair.is_ok());
        assert_eq!(correct_pair.unwrap().as_str(), correct_identifier);

        let incorrect_identifier = "1identifier";
        let incorrect_pair = ComposableFunctionParser::parse(
           Rule::identifier,
           incorrect_identifier,
        );
        assert!(incorrect_pair.is_err());

        let incorrect_identifier = "_identifier";
        let incorrect_pair = ComposableFunctionParser::parse(
           Rule::identifier,
           incorrect_identifier,
        );
        assert!(incorrect_pair.is_err());

        let incorrect_identifier = "identifier_";
        let incorrect_pair = ComposableFunctionParser::parse(
           Rule::identifier,
           incorrect_identifier,
        );
        assert!(incorrect_pair.is_ok());

        let incorrect_identifier = "iden tifier";
        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::identifier,
            incorrect_identifier,
        );
        assert!(incorrect_pair.is_ok());
    }
}