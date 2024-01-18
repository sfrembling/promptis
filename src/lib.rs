//! Simplify getting user input for your CLI applications
//!
//! Check out the examples, too:
//! - `cargo run --example closing` demonstrates quitting the program early
//! - `cargo run --example data` demonstrates getting data from the user
//! - `cargo run --example hello` is a basic hello world program

use std::io::{stdin, stdout, Write};

/// Handler for easily getting user input from the command line
#[derive(Debug, Default, Clone)]
pub struct Input {
    user_prompt: String,
    user_quit: Option<String>,
    user_errmsg: Option<String>,
}

impl Input {
    /// Create a new Input object to handle user input.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the prompt that will be displayed to the user.
    pub fn prompt(&mut self, p: &str) -> &mut Self {
        self.user_prompt = p.to_owned();
        self
    }

    /// Sets a phrase that, when entered, will end the program early.
    pub fn quit(&mut self, q: &str) -> &mut Self {
        self.user_quit = Some(q.into());
        self
    }

    /// Sets an error message that will be displayed to the user if they enter something invalid.
    pub fn err_msg(&mut self, m: &str) -> &mut Self {
        self.user_errmsg = Some(m.into());
        self
    }

    /// Waits until the user responds with something that can be parsed to `T`.
    ///
    /// If a `quit` trigger has been set and later read from the user, will exit early
    ///
    /// Example:
    /// ```
    /// let data: i32 = Input::new()
    ///     .prompt("Enter a number: ")
    ///     .wait();
    ///
    /// println!("Your number is {}", data);
    /// ```
    pub fn wait<T>(&self) -> T
    where
        T: std::str::FromStr,
    {
        let mut response = None;

        while let None = response {
            print!("{}", self.user_prompt);
            stdout().flush().unwrap();
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            if let Some(trigger) = &self.user_quit {
                if trigger == buffer.trim() {
                    std::process::exit(0);
                }
            }
            response = buffer.trim().parse().ok();
            if let None = response {
                if let Some(msg) = &self.user_errmsg {
                    println!("{}", msg);
                }
            }
        }

        response.unwrap()
    }

    /// Similar to `wait`, except will return after the user inputs anything.
    ///
    /// If the user input doesn't parse to `T`, `None` is returned.
    ///
    /// Example
    /// ```
    /// let data: Option<i32> = Input::new()
    ///     .prompt("Enter a number: ")
    ///     .read();
    ///
    /// match data {
    ///     Some(number) => println!("You entered {}", number),
    ///     None => println!("You didn't enter a number")
    /// }
    /// ```
    pub fn read<T>(&self) -> Option<T>
    where
        T: std::str::FromStr,
    {
        print!("{}", self.user_prompt);
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        if let Some(trigger) = &self.user_quit {
            if trigger == buffer.trim() {
                std::process::exit(0);
            }
        }
        buffer.trim().parse().ok()
    }
}
