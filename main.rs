use {std::io, std::thread, std::time::Duration};
mod commands{
    pub fn clear_console() {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("cls")
            .status();
    }
    pub fn ossze(fir:f64, sec:f64) -> f64{
        return fir + sec;
    }
    pub fn kiv(fir:f64, sec:f64) -> f64{
        return fir - sec;
    }
    pub fn oszt(fir:f64, sec:f64) -> f64{
        return fir / sec;
    }
    pub fn szor(fir:f64, sec:f64) -> f64{
        return fir * sec;
    }
}

fn main() {
    loop {
        let mut veg:f64 = 0.0;
        let mut my_vector: Vec<f64> = Vec::new();
        let mut user_input = String::new();
        //loop
        for _ in 0..2 {
            println!("Calculator\n\nEnter the a number: ");
            io::stdin().read_line(&mut user_input).expect("failed to readline");
            //pushback from user_input
            
            let u_float: f64 = match user_input.trim().parse() {
                Ok(num) => num, 
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
                
            };
            my_vector.push(u_float);
            commands::clear_console();
            user_input.clear();
        }
        user_input.clear();
        println!("First number : {}", my_vector[0]);
        println!("Second number : {}", my_vector[1]);
        println!("Enter a mathematical expresssion : ");

        //get math exp 
        io::stdin().read_line(&mut user_input).expect("failed to readline");
        if user_input.trim() == "+" {
            veg = commands::ossze(my_vector[0], my_vector[1]);
        }
        else if user_input.trim() == "-"{
            veg = commands::kiv(my_vector[0], my_vector[1]);
        }
        else if user_input.trim() == "*" {
            veg = commands::szor(my_vector[0], my_vector[1]);
        }
        else if user_input.trim() == ":" || user_input.trim() == "/" {
            veg = commands::oszt(my_vector[0], my_vector[1])
        }
        else {
            println!("Enter a valid choice!");
        }

        commands::clear_console();

        println!("Answer : {}", veg);
        
        thread::sleep(Duration::from_secs(4));
    }
}