fn main() {
    let mut i = 0;
    loop {
        if i == 10 { break };
        
        if i % 2 == 1 {
            i = i + 1;
            continue;
        }

        println!("i = {i}");

        i = i + 1;
    }

    println!("loop_return_value(): {}", loop_return_value());

    tuples();

    print_for_loop();

    print_range();
}

fn loop_return_value() -> i32 {
    let mut i = 0;

    loop {
        i += 1;

        if i == 10 {
            break i * 2;
        }
    }
}

fn tuples() {
    let x: (char, i32, f32) = ('a', 0, 0.0);

    println!("x.0 = {}", x.0);
}

fn print_for_loop() {
    let x = ['a', 'b', 'c', 'd'];

    for i in x {
        println!("i: {i}");
    }
}

fn print_range() {
    for i in 1..4 {
        println!("Range: {i}");
    }
}

