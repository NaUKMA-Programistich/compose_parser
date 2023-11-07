### Jetpack Compose Rust Parser

Link: https://crates.io/crates/compose_parser

* Rust parser created to parse the Jetpack Compose syntax tree
* It is important that the function starts with @Composable and has any name
* The main components will be
    * Row / Column -> elements that have another components as children
    * Text -> an element that has a text field parameter 'text'
    * Image -> an element that has a text field 'image' as a parameter with .png extension

### Technologies
* Rust and pest
* Created .pest file with grammar rules
* Created lib.rs file with parser

### First task

* Parse simple function like this
```kotlin
@Composable fun Example() {}
```

### Example

```kotlin
@Composable
fun Example() {
    Row {
        Text(text = "Hello")
        Column {
            Text(text = "World")
            Image(image = "image.png")
        }
    }
}
```

### Author

* Dzhos Oleksii