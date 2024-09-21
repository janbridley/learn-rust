const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The new value is {x}");


    // Shadowing: take use of variable name to current scope
    let x = x + 1; // x is now 7 in this scope

    {
        let x = x * 2; // x is 14 until this block ends
        println!("Inner scope x: {x}");
    }

    println!("Final value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // Shadow spaces to new type

    println!("Spaces prints as {spaces}.");
    

    // Cause integer overflow
    // let y: u8 = 256;


    let t = true;
    let f: bool = false;

    assert!(t!=f);


    let c = 'z';
    let z: char = 'ℝ';
    let heart_eyed_cat = '😻';


    let tup: (i32, f64, u8) = (500,1.234,99);


    let (x0,_,_) = tup;
    let x1 = tup.0;

    println!("The first value in our tuple is {x0}");
    println!("The first value in our tuple is {x1} (again)");

    let a = [1, 2, 3, 4, 5]; // stack allocated

    // Must be references to
    let days: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thurs", "Fri", "Sat"]; 

    // let x = days[8]; // Out of bounds! Will not compile
    
}
