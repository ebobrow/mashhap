mod mashhap;

fn main() {
    let mut map = mashhap::MashHap::new();
    map.set("A".to_string(), 1);
    map.set("Hello".to_string(), 5);
    println!("{:?}", map.get("A".to_string()));
    println!("{:?}", map.get("Hello".to_string()));
    println!("{:?}", map.get("nonexistant".to_string()));
    println!("{}", map.delete("A".to_string()));
    println!("{}", map.delete("B".to_string()));
    println!("{:?}", map.get("A".to_string()));
}
