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

fn recite(title: &String, play: &declarations::Play) {
    println!("Play: {}", title);
    
    //Used ChatGPT to understand the difference between &String and &str. I landed on using &str
    //because I can assign the string literal to it without needing to allocate heap space which is
    //more efficient.
    let mut cur_speaker: &str = "";
    for line in play {
        match line {
            (_num, name, text) => {
                if cur_speaker != name { //Swap characters
                    println!();
                    println!("{}.", name);
                    cur_speaker = name;
                }
                //Print line
                println!("{}", text);
            }
        }
    }

}


fn main() -> Result<(), u8>  {
    let mut config_file: String = Default::default();

    if let Err(e) = parse_args(&mut config_file){
        return Err(e);
    }
    
    let mut play_title: String = Default::default();
    let mut play: declarations::Play = Default::default();

    if let Err(e) = lab1::script_gen::script_gen(&config_file, &mut play_title, &mut play) {
        return Err(e);
    }

    play.sort();
    recite(&play_title, &play);
    Ok(())
}
