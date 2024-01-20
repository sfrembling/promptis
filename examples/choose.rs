use promptis::Input;

fn main() {
    let num = Input::new().choose("Are you sure you want to continue?");
    match num {
        true => println!("You continued"),
        false => println!("You didn't continue"),
    }
}
