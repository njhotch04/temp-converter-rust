//needs to be imported to handle the user input
use std::io;

//function to handle figuring out what the temperature scale is
fn temp_scale() -> String {
    //initial prompt for the user only asked once on startup
    println!("Are you currently measuring temperature in (C)elsius or (F)ahrenheit?");
    //a loop until we recieve a correct value from the user
    loop {
        //mutable variable declaring scale as a new string
        let mut scale = String::new();
        //this handles the user input and stores it in our scale variable using a reference
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line.");
        //trims the whitespace from our scale value and converts it to lowercase so we are easily able to match it
        let scale = scale.trim().to_lowercase();
        //use a match here instead of a loop or while to keep the code concise and clean
        match scale.as_str() {
            //if the user inputs Fahrenheit, f will be returned to the program
            "f" | "fahrenheit" => {
                println!("You have selected Fahrenheit.");
                return String::from("f");
            }
            //if the user inputs Celsius, c will be returned to the program
            "c" | "celsius" => {
                println!("You have selected Celsius.");
                return String::from("c");
            }
            //if the user makes an invalid selection this will continue to loop
            _ => println!("Invalid input. (F)ahrenheit or (C)elsius?"),
        }
    }
}

//function to handle identifying the number the user wants converted
fn temp_identify() -> f32 {
    //initial prompt for the user only asked once on initialization
    println!("What is the number you would like converted?");
    //a loop until we recieve a correct value from the user
    loop {
        //mutable variable declaring number as a new string
        let mut number = String::new();
        //this handles the user input and stores it in our number variable using a reference
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");
        //trims the whitespace from the input and converts the data type
        let number: f32 = match number.trim().parse() {
            Ok(num) => num,
            //curly brackets used to perform multiple actions
            Err(_) => {
                println!("Input invalid. Please try again.");
                continue;
            }
        };
        //now we return our discovered number value back to the rest of the program
        break number;
    }
}

//function to handle the actual conversion process to give back to the user
fn conversion(scale: String, number: f32) {
    //if the user selected Celsius we will use the Fahrenheit conversion
    if scale == "c" {
        let f = (number * 9.0 / 5.0) + 32.0;
        println!("{:.2}°C is equal to {:.2}°F.", number, f);
    //else we will use the Celsius conversion because there are only two usable scales
    } else {
        let c = (number - 32.0) * 5.0 / 9.0;
        println!("{:.2}°F is equal to {:.2}°C.", number, c);
    }
}

//the main program function where all of our functions will run together
fn main() {
    //first line to perform... title of the project
    println!("--- SUPER FAST TEMP CONVERTER ---");
    //grabs the value of temp_type
    let selected_scale = temp_scale();
    //grabs the value of temp_identify
    let selected_number = temp_identify();

    //now we pass our values to be completed by the conversion function
    conversion(selected_scale, selected_number);
}
