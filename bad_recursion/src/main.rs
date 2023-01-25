fn main() {
    let val = bad_rec(1);

    println!("bad_rec val: {val}");
}

fn bad_rec(x: i32) -> i32 {
    x + bad_rec(x + 1)
}

