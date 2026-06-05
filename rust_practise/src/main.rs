fn main() {

    let x = 5;
    println!("the value of x is: {}", x);

    {
        let x= 69;
        println!("in thhe inner scope the value of x is: {}", x);
    }

    let x=x+2;
    println!("the value of x now is: {}", x);

    {
        let x= "mhmdffhm";
        println!("x as a string within a scope is: {}", x);
    }

    let x= "ffffuhhhhyes";
        println!("x as a string is: {}", x);

    const MAX_X: i32 = 100000;

}

