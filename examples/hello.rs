use promptis::Input;

fn main() {
    let name: String = Input::new().prompt("Enter your name: ").wait();
    println!("Hello, {}!", name);
}
