use std::io;

// could be improved with caching

fn main() {
    println!("Welcome to fibonacci calculator");
    println!("Please enter a number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Couldn't read input!");

    let num:u8 = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };

    let output = fibonacci(num);    
    println!("Fibonacci output for num. {num} is: {output}");
}

fn fibonacci(num: u8) -> u32  {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1
    };

    return fibonacci(num-2)+fibonacci(num-1);
}
