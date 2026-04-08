use std::env;
use std::fs;
use sheller::run;
//use colored::*;
//use toml::Value;
//use once_cell::sync::Lazy;
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
//static configuration: Lazy<Value> = Lazy::new(||{
    //let tomlstr=fs::read_to_string("$HOME/.config/hrustfetch/config.toml").expect("Error read config file!");
    //toml::from_str(&tomlstr).expect("Error parsing config file!")
//});
fn knownos(){
    let content=fs::read_to_string("/etc/os-release")
        .expect("Error read /etc/os-release");
    let pretty_name=content
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .and_then(|line| line.split_once('='))
        .map(|(_, value)| value.trim_matches('"'))
        .unwrap();
    println!("Os: {}",pretty_name);
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
fn knowncpu(){
    let content = fs::read_to_string("/proc/cpuinfo").expect("Error read /proc/cpuinfo");
    if let Some(line) = content.lines().find(|l| l.contains("model name")) {
        let cpu_model = &line[13..];
        println!("Cpu: {}", cpu_model.trim());
    } else {
        println!("Error model name not found");
    }
}
fn knowngpu(){
    print!("Gpu: ");
    io::stdout().flush();
    sheller::run!("lspci | grep -i vga | cut -b 36- | sed 's/ *(rev.*)//'");
}
fn knownterm(){
    println!("Term: {}", env::var("TERM").unwrap_or_else(|_| "Unknown".to_string()));
}
fn knownhostname(){
    print!("Hostname: {}@", env::var("USER").unwrap_or_else(|_| "Unknown".to_string()));
    println!("{}",  env::var("HOSTNAME").unwrap_or_else(|_| "Unknown".to_string()));
}
fn blockcolors(){
    for i in 0..16{
        print!("\x1b[48;5;{}m  \x1b[0m",i);
    }
    println!();
}
fn knownfs(){
    print!("Filesystem: ");
    io::stdout().flush();
    sheller::run!("findmnt -n -o FSTYPE /");
}
fn main(){
    logo();
    knownos();
    knownkernel();
    knownshell();
    knownhostname();
    knownwm();
    knownterm();
    knownuptime();
    knowncpu();
    knowngpu();
    knownfs();
    blockcolors();
}
