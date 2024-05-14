use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [--help] [--random] [--base64] [--hex] [--check] [--checkp <password>] <length>", args[0]);
        process::exit(1);
    }

    let mut random = false;
    let mut base64 = false;
    let mut hex = false;
    let mut check = false;
    let mut checkp = false;
    let mut password_to_check = String::new();
    let mut length = 0;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => {
                println!("Usage: {} [--help] [--random] [--base64] [--hex] [--check] [--checkp <password>] <length>", args[0]);
                process::exit(0);
            }
            "--random" => random = true,
            "--base64" => base64 = true,
            "--hex" => hex = true,
            "--check" => check = true,
            "--checkp" => {
                checkp = true;
                i += 1;
                if i < args.len() {
                    password_to_check = args[i].clone();
                } else {
                    eprintln!("Error: --checkp requires a password argument");
                    process::exit(1);
                }
            }
            _ => length = args[i].parse().unwrap_or(0),
        }
        i += 1;
    }

    if checkp {
        check_password_strength(&password_to_check);
    } else {
        let password = if random {
            generate_password(length, "/dev/random", base64, hex)
        } else {
            generate_password(length, "/dev/urandom", base64, hex)
        };

        match password {
            Ok(password) => {
                println!("Generated password: {}", password);
                if check {
                    check_password_strength(&password);
                }
            }
            Err(e) => eprintln!("Error generating password: {}", e),
        }
    }
    Ok(())
}

fn generate_password(length: usize, device: &str, base64: bool, hex: bool) -> io::Result<String> {
    let mut file = File::open(device)?;
    let mut buffer = vec![0; length];
    file.read_exact(&mut buffer)?;

    if base64 {
        Ok(base64::encode(&buffer))
    } else if hex {
        Ok(hex::encode(&buffer))
    } else {
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~!@#$%^&*()";
        let mut password = String::new();
        for byte in buffer {
            let index = (byte as usize) % chars.len();
            password.push(chars.chars().nth(index).unwrap());
        }
        Ok(password)
    }
}

fn check_password_strength(password: &str) {
    let length_score = password.len() as u32;
    let mut digit_score = 0;
    let mut upper_score = 0;
    let mut lower_score = 0;
    let mut special_score = 0;

    for ch in password.chars() {
        if ch.is_digit(10) {
            digit_score = 1;
        } else if ch.is_uppercase() {
            upper_score = 1;
        } else if ch.is_lowercase() {
            lower_score = 1;
        } else {
            special_score = 1;
        }
    }

    let score = length_score + 5 * (digit_score + upper_score + lower_score + special_score);
    let (strength, color) = if score < 15 {
        ("weak", "\x1b[31m") // red
    } else if score < 25 {
        ("medium", "\x1b[33m") // yellow
    } else {
        ("strong", "\x1b[32m") // green
    };

    println!("Password strength: {}{} (score: {})\x1b[0m", color, strength, score);
}

