extern crate pest;
extern crate pest_derive;

use pest::{Parser, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "composable.pest"]
#[doc = r"Struct for parsing composable functions"]
pub struct ComposableFunctionParser;

#[derive(Debug, PartialEq, Eq)]
#[doc = r"Enum for composable function content"]
pub enum Content {
    FunctionName(String),
    Text(String),
    Image(String),
}

#[doc = r"Parse a composable function and return a vector of Content"]
pub fn parse_composable_content(input: &str) -> Result<Vec<Content>, pest::error::Error<Rule>> {
    let mut tokens = vec![];
    let parse_result = ComposableFunctionParser::parse(Rule::compose_function, input);

    return match parse_result {
        Ok(pairs) => {
            let composable_function = pairs
                .into_iter()
                .find(|pair| pair.as_rule() == Rule::compose_function)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No composable function found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            let function_declaration = composable_function
                .into_inner()
                .find(|pair| pair.as_rule() == Rule::function_declaration)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No function declaration found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            let function_declaration_pairs = function_declaration.into_inner();

            let function_name = function_declaration_pairs
                .clone()
                .find(|pair| pair.as_rule() == Rule::function_name)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No function name found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            tokens.push(Content::FunctionName(function_name.as_str().to_string()));

            let block_content = function_declaration_pairs
                .clone()
                .find(|pair| pair.as_rule() == Rule::block)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: "No block content found".to_string(),
                        },
                        Span::new(input, 0, 0).unwrap(),
                    )
                })?;

            for inner_pair in block_content.into_inner() {
                for statement in inner_pair.into_inner() {
                    match statement.as_rule() {
                        Rule::text_function => {
                            if let Some(text_function) = statement.into_inner().next() {
                                tokens.push(Content::Text(text_function.as_str().to_string()));
                            }
                        }
                        Rule::image_function => {
                            if let Some(image_function) = statement.into_inner().next() {
                                tokens.push(Content::Image(image_function.as_str().to_string()));
                            }
                        }
                        _ => {}
                    }
                }
            }
            return Ok(tokens);
        }
        Err(error) => Err(error),
    };
}
