use std::fmt::Display;

use promptis::Input;

fn main() {
    let choice = Input::new().quit("quit").wait_opts(
        &[Choice::Yes, Choice::No],
        &format!("Do you want to hear the dog speak?\nYour choice: "),
    );

    match choice {
        Choice::Yes => println!("Bark!"),
        Choice::No => println!("Awww ok :("),
    }
}

#[derive(Debug, Clone)]
enum Choice {
    Yes,
    No,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
