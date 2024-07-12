use core::panic;
use std::fs::File;
use std::io::{self, prelude::*, BufRead};
use std::path::Path;

fn main() {
    // test2();
    // test3();
    test4();
}
const LOREM_IPSUM : &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud";

fn test4() {
    let mut a = 0;
    if let Ok(lines) = read_lines("src/test01.txt") {
        for line in lines {
            a += 1;
            if let Ok(c) = line {
                println!("{}", c);
            }
        }
    }
    println!("{}", a);
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}
fn test3() {
    let p = Path::new("src/test01.txt");
    let d = p.display();

    let mut file = match File::create(&p) {
        Ok(file) => file,
        Err(e) => panic!("couldn't create {}: {:?}", d, e),
    };
    println!("{:?}", file);
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Ok(_) => println!("successfully wrote to {}", d),
        Err(e) => panic!("couldn't write to {}: {:?}", d, e),
    }
}
fn test2() {
    let p = Path::new("./src/hello.txt");
    let d = p.display();
    println!("{:?}", p);
    let mut file = match File::open(&p) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open {}: {:?}", d, err),
    };
    println!("{:?}", file);
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => println!("{} contains: \n{}", d, s),
        Err(e) => panic!("couldn't read {}: {:?}", d, e),
    }
}
fn test1() {
    let p = Path::new(".");
    let d = p.display();
    println!("{:?}", p);
    println!("{:?}", d);

    let new_path = p.join("a").join("b");
    println!("{:?}", new_path);
}
