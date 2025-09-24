//This file includes functions, types, and constants used for generating the play's script

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use super::declarations;

type PlayConfig = Vec<(String, String)>; // (character name, associated text file)
const TITLE_INDEX: usize = 0;
//const FIRST_CHARACTER_INDEX: usize = 1; The document told us to use this, but I just used a
//loop and after tracking the title index this becomes unnecessary
const CHARACTER_NAME: usize = 0;
const CHARACTER_FILE: usize = 1;
const CONFIG_LINE_TOKENS: usize = 2;
const MIN_CONFIG_LINES: usize = 2;
const TEST_APPEND: &str = "test_files/";


fn add_script_line(play: &mut declarations::Play, unparsed_line: &String, char_part_name: &String) {
    if unparsed_line.len() > 0 {
        if let Some((first_token, rest)) = unparsed_line.split_once(char::is_whitespace) {
            let first_token_trim = first_token.trim();
            let rest_trim = rest.trim();

            match first_token_trim.parse::<usize>() {
                Ok(num) => play.push((num, char_part_name.clone(), rest_trim.to_string())),
                Err(_) => {
                    use std::sync::atomic::Ordering;
                    if declarations::WHINGE_ON.load(Ordering::SeqCst) {
                        println!("Warning: {} does not contain a valid usize value", first_token_trim);
                    }
                },
            }

        }
    }
}


fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    let appended_path = format!("{}{}", TEST_APPEND, file_name);
    match File::open(appended_path) {
        Err(_) => {
            println!("Error: script generation failed because the file {} could not be opened", file_name);
            return Err(declarations::ERR_SCRIPT_GEN);
        },
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut s = String::new();
            loop {
                s.clear();
                match reader.read_line(&mut s) {
                    Err(_) => {
                        println!("Error: script generation failed because line could not be read");
                        return Err(declarations::ERR_SCRIPT_GEN);
                    },
                    Ok(bytes_read) => {
                        if bytes_read == 0 { //done reading
                            return Ok(())
                        }
                        file_lines.push(s.trim().to_string());
                    },
                }

            }
        },
    }
}


fn process_config(play: &mut declarations::Play, play_config: &PlayConfig) -> Result<(), u8> {
    for tup in play_config {
        match tup {
            (name, file) => {
                let mut lines: Vec<String> = Vec::new();
                grab_trimmed_file_lines(file, &mut lines)?; //Note: putting the ? causes a thrown error to propagate. This is the same as putting the function call in a match statement and returning the same error.
                for line in &lines {
                    add_script_line(play, line, name);
                }
            }
        }
    }
    Ok(())
}


fn add_config(line: &String, play_config: &mut PlayConfig) {
    let delimited_tokens: Vec<&str> = line.split_whitespace().collect();
    if delimited_tokens.len() != CONFIG_LINE_TOKENS {
        use std::sync::atomic::Ordering;
        if declarations::WHINGE_ON.load(Ordering::SeqCst) {
            println!("Warning: There were not exactly two distinct tokens in the line {}", line);
        }
    } else {
        play_config.push((
                delimited_tokens[CHARACTER_NAME].to_string(), 
                delimited_tokens[CHARACTER_FILE].to_string()
                ));
    }
}


fn read_config(config_file_name: &String, title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();
    grab_trimmed_file_lines(config_file_name, &mut lines)?;
    if lines.len() < MIN_CONFIG_LINES {
        return Err(declarations::ERR_SCRIPT_GEN);
    }
    for (i, line) in lines.iter().enumerate() {
        if i == TITLE_INDEX {
            *title = line.clone();
        } else {
            add_config(line, play_config);
        }
    }
    Ok(())
}


pub fn script_gen(config_file_name: &String, title: &mut String, play: &mut declarations::Play) -> Result<(), u8> {
    let mut play_config: PlayConfig = Default::default();
    read_config(config_file_name, title, &mut play_config)?;
    process_config(play, &play_config)?;
    Ok(())
}
