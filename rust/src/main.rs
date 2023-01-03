#![allow(unused)]
mod hec;
mod mashhap;

fn main() {
    let mut map = mashhap::MashHap::with_capacity(16);
    map.set("A", 1);
    map.set("Hello", 5);
    println!("{:?}", map.get("A"));
    println!("{:?}", map.get("Hello"));
    println!("{:?}", map.get("nonexistant"));
    println!("{}", map.delete("A"));
    println!("{}", map.delete("B"));
    println!("{:?}", map.get("A"));
}
