mod parser_tests {
    fn get_content_from_file(name: &str) -> String {
        let path = format!("examples/uncorrected/{}", name);
        return std::fs::read_to_string(path).expect("Unable to read file");
    }

    #[test]
    fn uncorrected_composable_annotation() {
        let content = get_content_from_file("uncorrected_composable_annotation.kt");
        let result = compose_parser::parse_composable_content(&content);
        assert!(result.is_err());
    }

    #[test]
    fn uncorrected_composable_without_fun() {
        let content = get_content_from_file("uncorrected_composable_without_fun.kt");
        let result = compose_parser::parse_composable_content(&content);
        assert!(result.is_err());
    }

    #[test]
    fn uncorrected_fun_name() {
        let content = get_content_from_file("uncorrected_fun_name.kt");
        let result = compose_parser::parse_composable_content(&content);
        assert!(result.is_err());
    }

    #[test]
    fn uncorrected_image_url() {
        let content = get_content_from_file("uncorrected_image_url.kt");
        let result = compose_parser::parse_composable_content(&content);
        assert!(result.is_err());
    }

    #[test]
    fn uncorrected_spaces() {
        let content = get_content_from_file("uncorrected_spaces.kt");
        let result = compose_parser::parse_composable_content(&content);
        assert!(result.is_err());
    }
}