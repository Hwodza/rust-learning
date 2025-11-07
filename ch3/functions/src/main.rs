fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");
    another_function(10);
    print_labeled_measurement(30, 'd');
    let x = five();
    println!("{x}");
}
