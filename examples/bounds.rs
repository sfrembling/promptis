use promptis::Input;

fn main() {
    let choice: Choice = Input::new()
        .quit("quit")
        .wait_opts(
            &["Yes", "No"],
            &format!("Do you want to hear the dog speak?\nYour choice: "),
        )
        .unwrap();

    match choice {
        Choice::Yes => println!("Bark!"),
        Choice::No => println!("Awww ok :("),
    }
}

enum Choice {
    Yes,
    No,
}

impl std::str::FromStr for Choice {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "YES" => Ok(Self::Yes),
            "NO" => Ok(Self::No),
            _ => Err(()),
        }
    }

    type Err = ();
}
