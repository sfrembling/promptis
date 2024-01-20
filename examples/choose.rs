use promptis::Input;

fn main() {
    let result = Input::new().choose("Are you sure you want to continue?");
    match result {
        true => println!("You continued"),
        false => println!("You didn't continue"),
    }

    if Input::new().choose("Erase everything?") {
        // ERASING EVERYTHING
        println!("Everything erased!");
    } else {
        println!("No action taken.");
    }
}
