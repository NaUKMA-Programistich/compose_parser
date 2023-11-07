extern crate pest_derive;
extern crate pest;

pub mod parser {
    use pest::Parser;
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "composable.pest"]
    #[doc = r"Struct for parsing composable functions"]
    struct ComposableFunctionParser;

    #[doc = r"Struct for parsing composable functions"]
    pub fn parse_composable_function(input: &str) -> Result<Vec<String>, pest::error::Error<Rule>> {
        let parser = ComposableFunctionParser::parse(Rule::compose_function, input);

        let mut tokens = vec![];

        match parser {
            Ok(pairs) => {
                for pair in pairs {
                    let result_parse = parse_composable_pair(pair);
                    match result_parse {
                        Ok(token) => {
                            tokens.push(token);
                        }
                        Err(error) => {
                            println!("Error on pair: {}", error);
                            return Err(error);
                        }
                    }
                }
                return Ok(tokens)
            }
            Err(error) => {
                println!("Error on parser: {}", error);
                return Err(error);
            }
        }
    }

    #[doc = r"Struct for parsing composable functions pair"]
    fn parse_composable_pair(pair: pest::iterators::Pair<Rule>) -> Result<String, pest::error::Error<Rule>> {
        return match pair.as_rule() {
            Rule::compose_function => {
                println!("Parsed Composable function: {:?}", pair.as_str());
                Ok(pair.as_str().to_string())
            }
            _ => {
                println!("Error on pair: {:?}", pair.as_str());
                Err(pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::CustomError {
                        message: format!("Error on pair: {:?}", pair.as_str())
                    },
                    pair.as_span()
                ))
            }
        }
    }
}