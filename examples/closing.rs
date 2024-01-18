use promptis::Input;

fn main() {
    let keyword = "quit";
    println!(
        "Enter the phrase '{}' to close the application early!",
        keyword
    );

    let _ = Input::new()
        .quit(keyword)
        .prompt("Enter: ")
        .wait::<String>();
    println!("Exiting normally - goodbye!");
}
