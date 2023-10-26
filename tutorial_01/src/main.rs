#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn greet() {
    println!("Your name:");

    let mut name = String::new();
    let greeting = "Nice to meet you";

    io::stdin().read_line(&mut name)
        .expect("Didn't receive name"); // Ok Error

    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn constants() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("age wasn't assigned a number");

    age += 1;

    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn main() {
    greet();
    constants();
}
