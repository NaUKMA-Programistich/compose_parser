### Jetpack Compose Rust Parser

Link: https://crates.io/crates/compose_parser

### Brief Describe

* Rust parser created to parse the Jetpack Compose Text / Image View and get all text fields and images
* It is important that the function starts with @Composable and has any name
* Plus use Kotlin syntax
  
### Grammar

```pest
compose_function = { function_declaration }
function_declaration = { "@Composable\nfun " ~ function_name ~ "() " ~ block }
function_name = { identifier }
block = { "{\n" ~ statement* ~ "}" }
statement = { text_function | image_function }
text_function = { "    Text(text = " ~ string ~ ")\n" }
image_function = { "    Image(image = " ~ string_image ~ ")\n" }
string = { "\"" ~ (ASCII_ALPHANUMERIC | " ")* ~ "\"" }
string_image = { "\"" ~ ASCII_ALPHANUMERIC* ~ ".png\"" }
identifier = { ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
```

compose_function - main function that contains all functions
function_declaration - function declaration with name and block
function_name - function name
block - block with statements, have or text or image
statement - text or image
text_function - text function with string
image_function - image function with string plus png prefix
string - string with text
string_image - string with png
identifier - function name

### Technologies
* Rust and pest
* composable.pest file with grammar rules
* Created lib.rs file with parser
* Created tests.rs file with test grammar rules
* Created cli.rs file with CLI

### Usage by CLI

```bash
./compose_parser -h or --help # show help
./compose_parser --file <path> # parse file (example.txt)
./compose_parser --author # show author

./compose_parser without any arguments # show help
```

### Example

```kotlin
./compose_parser --file example.kt

@Composable
fun Example() {
    Text(text = "World")
    Image(image = "url.png")
    Image(image = "image.png")
}

// In result you have function name and text and image fields 

Function: Example
Text: World
Image: url.png
Image: image.png
```

### Author

* Dzhos Oleksii me@programistich.com