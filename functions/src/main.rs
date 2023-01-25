fn main() {
    print_labeled_measurement(5, 'h');
    println!("{}", recfunc(5));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}.");
}

fn recfunc(x: i32) -> i32 {
    if (x == 0) {
        return 0;
    }
    x + recfunc(x-1)
}

