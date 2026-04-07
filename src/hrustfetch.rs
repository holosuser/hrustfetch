use std::env;
use std::fs;
use sheller::run;
use colored::*;
use std::io::{self,Write};
fn logo() {
    let content=fs::read_to_string("/etc/os-release").expect("Error read /etc/os-release");
    let mut osn=String::new();
    for line in content.lines(){
        if line.starts_with("ID="){
            osn=line.replace("ID=", "").replace("\"", "");
            break;
        }
    }
    match osn.as_str(){
        "arch" => run!("viu -t -w 40 -h 20 /etc/hrustfetch/assets/arch.png"),
        "gentoo" => run!("viu -t -w 40 -h 20 /etc/hrustfetch/assets/gentoo.png"),
        "nixos" => run!("viu -t -w 40 -h 20 /etc/hrustfetch/assets/nixos.png"),
        "debian" => run!("viu -t -w 40 -h 20 /etc/hrustfetch/assets/debian.png"),
        "void" => run!("viu -t -w 40 -h 20 /etc/hrustfetch/assets/void.png"),
        _ => println!("unknown distro"),
    }
}
fn knownos(){
    let content=fs::read_to_string("/etc/os-release")
        .expect("Error read /etc/os-release");
    let pretty_name=content
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .and_then(|line| line.split_once('='))
        .map(|(_, value)| value.trim_matches('"'))
        .unwrap();
    println!("Os: {}", pretty_name);
}
fn knownkernel(){
    print!("Kernel: ");
    io::stdout().flush();
    sheller::run!("uname -rs");
}
fn knownshell(){
    println!("Shell: {}", env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string()));
}
fn knownwm(){
    println!("Wm: {}", env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string()));
}
fn knownuptime(){
    print!("Uptime: ");
    io::stdout().flush();
    sheller::run!("uptime -p");
}
fn main() {
    logo();
    knownos();
    knownkernel();
    knownshell();
    knownwm();
    knownuptime();
}
