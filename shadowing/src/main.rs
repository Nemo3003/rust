

fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{}", x); // Prints "42".

    removed()
}


// Remove a line in the code to make it compile
fn removed() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x += 3;


    let y: i32 = 4;
    // Shadowing
    let y: &str = "I can also be bound to text!"; 

    println!("Success!");
}