#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::{Error, Read};
use std::net::IpAddr;

fn main() {
    // panic();

    // another_panic();

    // open_file();

    // open_file_2();

    // let result: Result<String, Error> = read_username_from_file();

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    let home2: IpAddr = "127.0.0.1-abc".parse().unwrap();
}

fn panic() {
    panic!("Frauen und Kinder zu erst!");
}

fn another_panic() {
    let v = vec![1, 2, 3];

    v[99];
}

fn open_file() {
    let f = File::open("hello.txt"); // f: Result<std::fs::File, std::io::Error>

    let x = match f {
        Ok(file) => file, // file: std::fs::File
        Err(error) => { // error: std::io::Error
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

fn open_file_2() {
    let f = File::open("hello.txt").expect("Konnte hello.txt nicht öffnen");
}

fn read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?; // Beachte das ? am Ende
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Beachte das ? am Ende
    Ok(s)
}
