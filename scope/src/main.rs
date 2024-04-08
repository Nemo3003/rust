fn main(){
    let x: i32 = 10;
    let y: i32 = 5;

    {
        println!("The value of x is {} and value of y is {}", x, y);
    }

    println!("The value of x is {} and value of y is {}", x, y);
    
    define_x()
}

fn define_x() {
    let x: &str = "hello";
    println!("The value of x is {}", x);


}