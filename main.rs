
use {std::time::Duration, std::fs::OpenOptions, std::io::Write, std::io};
mod calc{
    pub fn ossz(fir:f64,sec:f64) -> f64 {
        return fir + sec;
    }

    pub fn kiv(fir:f64,sec:f64) -> f64 {
        return fir - sec;
    }
    pub fn oszt(fir:f64,sec:f64) -> f64 {
        return fir / sec;
    }
    pub fn szor(fir:f64,sec:f64) -> f64 {
        return fir * sec;
    }
}
mod etc{
    pub fn clear_console(is:bool) {
        let _ = std::process::Command::new("cmd")
            .arg("/C")
            .arg("cls")
            .status();
        if is {println!("Calculator\n");}
    }   
}

fn main(){
    loop {
        etc::clear_console(false);
        let mut userfloat :Vec<f64> = Vec::new();
        let mut userlog :Vec<String> = Vec::new();
        let mut user_input = String::new();
        let veg:f64;
        println!("Calculator\n");
        for _ in 0..2 {
            
            println!("Enter a number :");
            io::stdin().read_line(&mut user_input).expect("Couldnt read line!");

            let u_float:f64 = match user_input.trim().parse() {
                Ok(u_float) => u_float,
                Err(_) => {
                    println!("Enter a vaild number!");
                    return;
                }
            };
            //pushback logs
            userfloat.push(u_float);
            userlog.push(u_float.to_string());
            user_input.clear();
            etc::clear_console(true);
        }
        println!("Current number you are working with : \n{}\n{}\n", userfloat[0], userfloat[1]);
        println!("Enter a math. expression");
        io::stdin().read_line(&mut user_input).expect("Couldnt read line!");
        if user_input.trim() == "+" {
            veg = calc::ossz(userfloat[0],userfloat[1]);
        }
        else if user_input.trim() == "-" {
            veg = calc::kiv(userfloat[0],userfloat[1]);
        }
        else if user_input.trim() == "*" {
            veg = calc::szor(userfloat[0],userfloat[1]);
        }
        else if user_input.trim() == "/" || user_input.trim() == ":" {
            veg = calc::oszt(userfloat[0],userfloat[1]);
        }
        else {
            println!("Enter a valid expression!");
            return;
        }
        etc::clear_console(true);

        //pushback veg and exp input + edit them
        userlog.push(user_input.trim().to_string());
        userlog.push("=".to_string());
        userlog.push(veg.to_string());
        userlog.swap(1, 2);
        
        //vec::userlog => 0. 1num 1. 2num 3. exp 4.veg
        //open and write to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("math.history")
            .expect("Failed to open file");

        for index in 0..userlog.len() {
            match write!(file, "{} ", userlog[index]){
            Ok(_) => {},
            Err(e) =>
                {
                    println!("Error writing to the file!\n{}", e);
                }
            }
        }
        write!(file, "\n").expect("");
        //after writing finish up
        

        //after every exp poss. print veg
        println!("Answer : {}\n\n", veg);
        std::thread::sleep(Duration::from_secs(4));
    }
}
