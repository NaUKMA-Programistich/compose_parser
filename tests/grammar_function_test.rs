mod tests {
    use pest::Parser;
    use compose_parser::{ComposableFunctionParser, Rule};

    #[test]
    // text_function = { "    Text(text = " ~ string ~ ")\n" }
    fn test_text_function() {
        let input = "    Text(text = \"World\")\n";
        let correct_pair = ComposableFunctionParser::parse(Rule::text_function, input);

        assert!(correct_pair.is_ok());
        assert_eq!(correct_pair.unwrap().as_str(), input);

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::text_function,
            "    Text(text = \"World\")"
        );
        assert!(incorrect_pair.is_err());

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::text_function,
            "Text(text = \"World\")\n"
        );
        assert!(incorrect_pair.is_err());
    }

    #[test]
    // image_function = { "    Image(imageUrl = " ~ string ~ ")\n" }
    fn test_image_function() {
        let input = "    Image(image = \"url.png\")\n";
        let correct_pair = ComposableFunctionParser::parse(Rule::image_function, input);

        assert!(correct_pair.is_ok());


        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::image_function,
            "    Image(image = \"url.png\")"
        );
        assert!(incorrect_pair.is_err());

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::image_function,
            "Image(image = \"url.png\")\n"
        );
        assert!(incorrect_pair.is_err());

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::image_function,
            "Image(imageUrl = \"url.png\")\n"
        );
        assert!(incorrect_pair.is_err());

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::image_function,
            "Image(imageUrl = \"url.png)\n"
        );
        assert!(incorrect_pair.is_err());
    }

    #[test]
    // statement = { text_function | image_function }
    fn test_statement() {
        let input = "    Text(text = \"World\")\n";
        let correct_pair = ComposableFunctionParser::parse(Rule::statement, input);
        assert!(correct_pair.is_ok());

        let input = "    Image(image = \"url.png\")\n";
        let correct_pair = ComposableFunctionParser::parse(Rule::statement, input);
        assert!(correct_pair.is_ok());

        let incorrect_pair = ComposableFunctionParser::parse(
            Rule::statement,
            "    Text(text = \"World\")"
        );
        assert!(incorrect_pair.is_err());
    }
}