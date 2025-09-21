pub mod lab1;

use std::env;
use lab1::declarations;


fn usage(name: &String) {
    println!("Usage: ./{name} <configuration_file_name> [whinge]");
}

fn parse_args(name: &mut String) -> Result<(), u8> {
    let mut args = Vec::<String>::new();
    for arg in env::args() {
        args.push(arg);
    }
    
    //Check if valid input
    if args.len() < declarations::MIN_ARGS  || 
    args.len() > declarations::MAX_ARGS || 
    (args.len() == declarations::MAX_ARGS && args[declarations::WHINGE_MODE] != "whinge".to_string()){

        usage(&args[declarations::PROG_NAME]);
        return Err(declarations::ERR_CMD_LINE);
    }

    *name = args[declarations::CONFIG_FILE].clone(); //The clone is because the vector owns the string and giving it to name would pass ownership and put the vector in an unknown state. I used ChatGPT to understand this error more thoroughly and understand why .clone() is a good solution.
    
    if args.len() == declarations::MAX_ARGS {
        use std::sync::atomic::Ordering;
        declarations::WHINGE_ON.store(true, Ordering::SeqCst); 
    }
    Ok(())
}


fn main() -> Result<(), u8>  {
    let mut s: String = "ex string".to_string();
    parse_args(&mut s);
    println!("whinge: {:?}", declarations::WHINGE_ON);
    Ok(())
}
