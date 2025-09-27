CSE5402 Fall 2025 - Lab 1
Name: Daniel Palmer
Email: d.m.palmer@wustl.edu


1. Summary

The overall approach that I took is incredly straightforward following the instructions for the assignment. I first created the declarations file that holds program-wide types, constants, and static variables. I then created helper functions for telling users how to run the program when improper command line arguments are used and for printing the play once all of the lines have been constructed. I then created the functions that perform the core script generation, including adding lines to the script, parsing files for lines, parsing the play's config type which holds each character's name and text files, adding a tuple of name and file to the config type, creating the config type from the config file, and generating the script using all of the previous functions. Finally, I created the main function which processes the command line arguments, creates the play, sorts the play by line order, and prints the play all using aforementioned helper functions. 

2. Modules

I set up the module structure by creating a core subdirectory titled "lab1" that holds all of the program's functionality other than environment-related tasks such as command line arguments. Within this subdirectory, I created two files, one that contains types, constants, and static variables used across the entire program and another that contains the core routine for generating the script. I import this module into my main file which is also where I put all of the functions that handle environment-related tasks and the main function that executes the program. 

3. Insights/Observations

I had a few insights that required more research as I was doing this project.
- Understanding when to clone a string, particularly when the string is owned by a vector. 
- Understanding the difference between &String and &str. This came up when creating my recite function because I don't need to allocate heap space for tracking the current speaker. In this instance I used &str because it was more efficient.
- Understanding the best way to propagate errors. I used ChatGPT to understand the different methods, finally landing on using a terminating ? after a call that returns a Result<(), u8>. This question mark just propagates the error out if it happened which is a very efficient syntactic way to manage nested errors. 

4. Decisions Straying From Instructions

I decided that if the add_script_line function runs into a line that contains only a single token and whinge mode is on it should complain about this line. Given that it complains about improper line numbering, if the line only contains a single token it should whinge while not adding the line.

I also decided that if the play does not contain any valid lines a message saying as much should be printed in addition to the play's title. This is so that the user knows what exactly happened. 

_________________________________________________________________________________

Instructions: 

I have zipped my entire program into a file titled palmer_5402_lab_1.zip. I have organized my test files into a subdirectory titled test_files which contains the config file and any associated character files that are in the config file. This unzipped directory also includes an example output file which is the provided hamlet file. 

So my program can be ran with the commands

unzip palmer_5402_lab_1.zip
cd lab1
cargo run <configuration_file_name> [whinge]

_________________________________________________________________________________

Testing: 

I have tested my program under a wide variety of conditions. 
- First, I tested it using fully welly formed input which succeeded. 
- I then tested it by adding much whitespace to the character files, which succeeded without triggering whinging. This is expected behavior because whitespace should not impact output. 
- I then tested it by adding lines without valid line numbers which triggered whinging because the line is invalid, but otherwise had the program continue. 
- I then tested it by adding lines that contain only a single token which whinge if desired and don't add the line. 
- I then tested the function with both config files that don't exist and character files that don't exist which result in the program returning an error code indicating script generation failed. 
- I then tested with a config file that did not contain any characters, which resulted in the script generation fail error which is expected. 
- I then tested with lines in the config file that contained 1 or more than 2 tokens, which whinged but allowed the program to continue. 
- Finally, I tested the program with bad command line arguments including too many, too few, and a third argument that is not "whinge". All of these resulted in the associated bad command line error code. 

Overall, the program runs exactly as desired. It puts together the play from the provided character files which outputs a line even if there is a single correct line in the entire.

I have included within the test_files subdirectory the example hamlet files along with a file that contains badly formed inputs that should whinge while still succeeding to produce an output. 
