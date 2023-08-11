fn main() {
    println!("Hello, world!");

    let x = 10;
    print!("x: {}", x);

    let number = 42;

    if number < 0 {
        print!("The numbers is negative.");
    } else if number > 0 {
        panic!("The number is positive.");
    } else {
        print!("The number is zero.");
    }

    //looping from 1 to 5
    for i in 1..=5{
        print!("Loop iteration: {}", i);
    }
}
