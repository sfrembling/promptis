use promptis::Input;

fn main() {
    let mut inp = Input::new();
    let id = inp.prompt("Enter a number: ").read::<i32>();

    match id {
        Some(number) => println!("Your number is {}", number),
        None => println!("That wasn't a number!"),
    }
}
