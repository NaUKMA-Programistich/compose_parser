mod tests {
    use compose_parser::{ComposableFunctionParser, Rule};
    use pest::Parser;

    #[test]
    // block = { "{\n" ~ statement* ~ "}" }
    fn test_block() {
        let input = "{\n    Text(text = \"World\")\n}";
        let correct_pair = ComposableFunctionParser::parse(Rule::block, input);
        assert!(correct_pair.is_ok());

        let input = "{\n    Text(text = \"World\")\n    Text(text = \"Hello\")\n}";
        let correct_pair = ComposableFunctionParser::parse(Rule::block, input);
        assert!(correct_pair.is_ok());

        let input = "{\t    Text(text = \"World\")\n    Text(text = \"Hello\")\n}";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::block, input);
        assert!(incorrect_pair.is_err());

        let input = "{\t    Text(text = \"World\")\n    Text(text = \"Hello\")\n";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::block, input);
        assert!(incorrect_pair.is_err());
    }

    #[test]
    // function_declaration = { "@Composable\nfun " ~ function_name ~ "() " ~ block }
    fn test_function_declaration() {
        let input = "@Composable\nfun Example() {\n    Text(text = \"World\")\n}";
        let correct_pair = ComposableFunctionParser::parse(Rule::function_declaration, input);

        assert!(correct_pair.is_ok());

        let input =
            "@Compose\nfun Example() {\n    Text(text = \"World\")\n    Text(text = \"Hello\")\n}";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::function_declaration, input);
        assert!(incorrect_pair.is_err());

        let input = "@Composable\nfun Example() {\n    Text(text = \"World\")\n    Text(text = \"Hello\")\n";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::function_declaration, input);
        assert!(incorrect_pair.is_err());

        let input =
            "@Composable\nfn Example() {\n    Text(text = \"World\")\n    Text(text = \"Hello\")\n";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::function_declaration, input);
        assert!(incorrect_pair.is_err());

        let input = "@Composable\nfun Exa mple() {\n    Text(text = \"World\")\n    Text(text = \"Hello\")\n";
        let incorrect_pair = ComposableFunctionParser::parse(Rule::function_declaration, input);
        assert!(incorrect_pair.is_err());
    }
}
