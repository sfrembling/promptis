use promptis::Input;

fn main() {
    let num_mats: usize = Input::new().prompt("# of materials: ").wait();
    let mut data = Vec::new();
    let mut input = Input::new()
        .quit("quit")
        .err_msg("Unexpected input, please retry");
    for _ in 0..num_mats {
        let mat: String = input.prompt("Material ID: ").wait();
        let quantity: f64 = input.prompt("Quantity: ").wait();
        let unit: String = input.prompt("Unit of Measure: ").wait();
        data.push((mat, quantity, unit));
    }

    for dp in data {
        println!("Mat: {} - Quantity: {} - UoM: {}", dp.0, dp.1, dp.2);
    }
}
