// use std::env;
use std::io::{stdin}; //, stdout, Write};
// use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    //read_line leaves a trailing newline, which trim removes
    let command = input.trim();

    Command::new(command)
        .spawn()
        .unwrap();
}