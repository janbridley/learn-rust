fn main() {
    println!("Hello, world!");

    let _temp = 123;

    let result = loop { // Infinite loop without break
        println!("again!");
        break _temp * 2;
    };
    println!("Result = {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count +=  1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Counted down!");
    assert!(number == 0);

    let a = [1,2,3,4,5];

    for element in a {
        println!("Element: {element}")
    }

    for num in (1..=5).rev(){
        println!("{num}!")
    }
    println!("liftoff!")

}
