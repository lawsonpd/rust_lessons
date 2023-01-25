fn main() {
    let s1 = String::from("a string");
    let s2 = s1;

    // This causes a compile-time error, since s1 is 
    // considered by the compiler to have been "moved" to s2
    // println!("s1 is {s1}"); 

    println!("s2 is {s2}");

    out_of_scope();

    //
    // The following shows what happens when you try to use a value
    // that has been moved into the scope of (i.e. is owned by)
    // another function.
    //

    let x = String::from("another string");

    new_owning_fn(x);

    // println!("x is {x}"); // Compiler throws an error for this

    let y1 = String::from("yet another string");
    let y2 = return_ownership(y1);
}

fn out_of_scope() {
    let x = 0;

    if x == 0 {
        println!("x == 0");

        // let y = 1;
    }

    // println!("y == {y}"); // y is out of scope here and will cause a compile-time exception
}

fn new_owning_fn(var: String) {
    println!("var is {var}");
}

fn return_ownership(var: String) -> String {
    String::from(var)
}

