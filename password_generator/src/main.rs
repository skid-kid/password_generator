use std::io;
use rand::{Rng,distributions::Alphanumeric};
fn generate_random_password(length: usize)->String{
    let mut rng=rand::thread_rng();
    let password :String=(0..length)
     .map(|_| rng.sample(Alphanumeric) as char)
     .collect();
    password 
}
fn main() {
    println!("Enter the length of password you want to generate:");
    let mut password_length=String::new();
    io::stdin().read_line(&mut password_length).expect("Failed to read line");
    let password:usize=match password_length.trim().parse(){
        Ok(num) => num,
        Err(_) => 8,
    };
    let password=generate_random_password(password);
    println!("Random password:{}",password);
    }

