mod parser_tests {

    #[test]
    fn parser_correct() {
        let input = "@Composable fun Example() {}";
        let parser = compose_parser::parser::parse_composable_function(input);
        println!("{:?}", parser);

        assert!(parser.is_ok());
        let unwrapped = parser.unwrap();
        assert_eq!(unwrapped[0], input);
    }

    #[test]
    fn parser_not_function_declaration() {
        let input = "@Composable fu Example() {";
        let parser = compose_parser::parser::parse_composable_function(input);
        println!("{:?}", parser);

        assert!(parser.is_err());
    }

    #[test]
    fn parser_not_function_name() {
        let input = "@Composable fun () {}";
        let parser = compose_parser::parser::parse_composable_function(input);
        println!("{:?}", parser);

        assert!(parser.is_err());
    }

    #[test]
    fn parser_not_function_name_with_number() {
        let input = "@Composable fun 123() {}";
        let parser = compose_parser::parser::parse_composable_function(input);
        println!("{:?}", parser);

        assert!(parser.is_err());
    }
}