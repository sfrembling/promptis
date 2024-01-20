use promptis::Input;

fn main() {
    let message: String = Input::new().prompt("Enter a message: ").wait();
    let repeat: usize = Input::new()
        .err_msg("That wasn't a number... try again")
        .prompt("Enter the number of times to repeat the message: ")
        .wait();
    for i in 0..repeat {
        println!("{}. {}", i + 1, message);
    }
}
