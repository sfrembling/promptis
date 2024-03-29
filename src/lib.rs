//! Simplify getting user input for your CLI applications
//!
//! Check out the examples, too:
//! - `cargo run --example closing` demonstrates quitting the program early
//! - `cargo run --example data` demonstrates getting data from the user
//! - `cargo run --example hello` is a basic hello world program
//!
//! Example usage:
//! ```
//! let name: String = Input::new()
//!     .prompt("Enter your name: ")
//!     .wait();
//!
//! println!("Hello, {}!", name);
//! ```
//!
//! You can also set error messages for when the user messes up the input.
//! ```
//! let number: i32 = Input::new()
//!     .err_msg("That wasn't a number; please try again")
//!     .prompt("Enter a number: ")
//!     .wait();
//!
//! println!("Your number is: {}", number);
//! ```
//!
//! You can choose to just get the first input, regardless of whether it's good.
//! ```
//! let number: Option<i32> = Input::new()
//!     .prompt("Enter a number: ")
//!     .read();
//!
//! match number {
//!     Some(n) => println!("Your number is: {}"),
//!     None => println!("You didn't enter a number!")
//! }
//! ```
//!
//! You can specify a keyword that will end the program when entered
//! ```
//! let number: i32 = Input::new()
//!     .quit("quit") // this can result in the program ending early
//!     .prompt("Enter a number: ")
//!     .wait();
//!
//! println!("Your number is: {}", number);
//! ```
//!
//! You can re-use the same input object for multiple inputs.
//! ```
//! let mut input = Input::new()
//!     .err_msg("Unexpected input; please retry")
//!     .quit("quit");
//!
//! let name: String = input.prompt("Enter your name: ").wait();
//! let age: u32 = input.prompt("Enter your age: ").wait();
//! let weight: f64 = input.prompt("Enter your weight: ").wait();
//!
//! println!("Name: {}\nAge: {}\nWeight: {}", name, age, weight);
//! ```

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
    pub fn quit(mut self, q: &str) -> Self {
        self.user_quit = Some(q.into());
        self
    }

    /// Sets an error message that will be displayed to the user if they enter something invalid.
    pub fn err_msg(mut self, m: &str) -> Self {
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

        while response.is_none() {
            response = self.get_data();
            self.check_error(&response);
        }

        // At this point we know that this holds a value
        // so unwrapping should be fine.
        response.unwrap()
    }

    /// Checks if the user's input is the quit trigger, and if so, ends the program
    fn check_quit(&self, message: &str) {
        if let Some(trigger) = &self.user_quit {
            if trigger == message.trim() {
                std::process::exit(0);
            }
        }
    }

    /// Checks whether `response` was entered incorrectly, and if so, prints the error message
    fn check_error<T>(&self, response: &Option<T>) {
        if response.is_none() {
            if let Some(msg) = &self.user_errmsg {
                println!("{}", msg);
            }
        }
    }

    /// Handles getting data from the user
    fn get_data<T>(&self) -> Option<T>
    where
        T: std::str::FromStr,
    {
        print!("{}", self.user_prompt);
        self.handle_io(|| stdout().flush());
        let mut buffer = String::new();
        self.handle_io(|| stdin().read_line(&mut buffer));
        self.check_quit(&buffer);
        buffer.trim().parse().ok()
    }

    /// Handles [std::io] operations; will simply print that an error
    /// occurred and continue on
    fn handle_io<T, F>(&self, mut io: F)
    where
        F: FnMut() -> std::io::Result<T>,
    {
        if let Err(e) = io() {
            println!("IO Error: {}; Continuing...", e);
        }
    }

    /// Presents a series of options to the user from which they can choose one.
    ///
    /// This function will guarantee that the user chooses something present in `opts`
    ///
    /// This function can return `Err` if the user's option doesn't parse into `T`
    ///
    /// Example:
    /// ```
    /// let choice: String = Input::new()
    ///     .wait_opts(&["First", "Second", "Third"], "Enter your choice: ")
    ///     .unwrap();
    ///
    /// match choice.as_str() {
    ///     "First" => println!("1st!"),
    ///     "Second" => println!("2nd"),
    ///     "Third" => println!("3rd..")
    /// }
    /// ```
    ///
    /// The user in the above case would see the following:
    /// ```markdown
    /// 1. First
    /// 2. Second
    /// 3. Third
    /// Enter your choice:
    /// ```
    pub fn wait_opts<T>(&self, opts: &[T], p: &str) -> T
    where
        T: std::fmt::Display + Clone,
    {
        let index;

        // This is so that the input object will respect err_msg rules and quit triggers
        let mut ic = self.clone();

        loop {
            for (i, v) in opts.iter().enumerate() {
                println!("{}. {}", i + 1, v);
            }

            let result = ic.prompt(p).wait();

            if (1..=opts.len()).contains(&result) {
                index = result - 1;
                break;
            } else {
                println!(
                    "Please enter a number within the bounds {:?}",
                    1..=opts.len()
                );
            }
        }

        opts[index].clone()
    }

    /// Presents a simple "yes/no" option to the user, returning their choice
    ///
    /// This is useful for binary decisions, i.e. asking for confirmation before progressing
    ///
    /// Example:
    /// ```
    /// if Input::new().choose("Continue?") {
    ///     println!("You continued.");
    /// }
    /// ```
    /// What the user would see:
    /// ```plaintext
    /// Continue? [y/n]
    /// ```
    pub fn choose(&self, p: &str) -> bool {
        loop {
            let inp: char = self.clone().prompt(&format!("{} [y/n] ", p)).wait();
            match inp.to_ascii_uppercase() {
                'Y' => {
                    return true;
                }
                'N' => {
                    return false;
                }
                _ => {}
            }
        }
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
        self.get_data()
    }
}
