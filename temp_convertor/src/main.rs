use std::io;

fn main(){
    println!("Welcome to temperature convertor!");
    println!("Select 1. to convert from Fahrenheit to Celsius.");
    println!("Select 2. to convert from Celsius to Fahrenheit.");

    let mut sel_num = String::new();

    io::stdin()
        .read_line(&mut sel_num)
        .expect("Couldn't read input!");

    let sel_num: u32 = match sel_num.trim().parse(){
        Ok(sel_num) => sel_num,
        Err(_) => {
            println!("Please enter a number!");
            return;
        },
    };

    if sel_num != 1 && sel_num != 2 {
        println!("Enter either 1 or 2.");
        return
    };

    println!("Please input the temperature:");
    let mut temp = String::new();
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Couldn't read intpu!");

    let temp: f32 = match temp.trim().parse(){
        Ok(temp) => temp,
        Err(_) => {
            println!("Please enter a number!");
            return;
        }
    };

    let fahr_wanted: bool = if sel_num == 2 {true} else {false};
    let output: f32 = if fahr_wanted {cel_to_fahr(temp)} else {fahr_to_cel(temp)};
    let unit = if fahr_wanted {"°F"} else {"°C"};
    println!("Converted temperatur: {output}{unit}");
}

fn cel_to_fahr(celsius: f32) -> f32 {
    (celsius*1.8) + 32.0
}

fn fahr_to_cel(fahrenheit: f32) -> f32{
    (fahrenheit-32.0)/1.8
}