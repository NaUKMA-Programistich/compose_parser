mod parser_tests {
    use compose_parser::Content;

    fn get_content_from_file(name: &str) -> String {
        let path = format!("examples/correct/{}", name);
        return std::fs::read_to_string(path).expect("Unable to read file");
    }

    #[test]
    fn parser_correct_only_one_text() {
        let input = get_content_from_file("parser_correct_only_one_text.kt");
        let result = compose_parser::parse_composable_content(&input);
        let expected = vec![
            Content::FunctionName("Example".to_string()),
            Content::Text("\"World\"".to_string()),
        ];

        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual.len(), 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parser_correct_only_one_image() {
        let input = get_content_from_file("parser_correct_only_one_image.kt");
        let result = compose_parser::parse_composable_content(&input);
        let expected = vec![
            Content::FunctionName("Example".to_string()),
            Content::Image("\"url.png\"".to_string()),
        ];

        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual.len(), 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parser_correct_multiply() {
        let input = get_content_from_file("parser_correct_multiply.kt");
        let result = compose_parser::parse_composable_content(&input);
        let expected = vec![
            Content::FunctionName("ExampleTest".to_string()),
            Content::Text("\"World\"".to_string()),
            Content::Text("\"World Test\"".to_string()),
            Content::Image("\"url.png\"".to_string()),
            Content::Text("\"World Test Parser\"".to_string()),
        ];

        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual.len(), 5);
        assert_eq!(actual, expected);
    }
}
