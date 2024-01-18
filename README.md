# promptis
A Rust crate to simplify getting user input in the CLI.

## Examples
`cargo run --example hello`: A basic Hello World program that reads the user's name

`cargo run --example data`: A program that asks for a message and a number and repeats the message that many times

`cargo run --example closing`: A program that demonstrates the user ending the program early with an input

## Example Usage
```rust
// Prompt for the user's name and wait for them to respond with input
let name: String = Input::new().prompt("Enter your name: ").wait();

// Prompt the user for a number and wait for them to respond, 
// displaying the error if they input something else
let id: u32 = Input::new()
    .prompt("Enter a number: ")
    .err_msg("Not a number; please retry")
    .wait();
```