use compose_parser::parser;

fn main() {
    let input = "@Composable fun Example() {}";
    let parser = parser::parse_composable_function(input);
    println!("{:?}", parser)
}

