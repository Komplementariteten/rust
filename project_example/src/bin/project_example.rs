extern crate project_example;

use project_example::sub::file1::*;

fn main() {
    println!("main");
    hello();
}

/*
type Error = Box<dyn error::Error>;
fn print() {
    let n = '🤬';
    println!("hallo welt {}", n);
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("abc",1);
    let c = m.iter().filter(| &(k, v) | *v == 1).count();
    println!("{}", c);

    let t: (i16, i16, i16) = (1, 2543, 34);
    let rang = 0..=255;
    let mut t1 = rang.contains(&t.0);
    t1 &= rang.contains(&t.1);
    t1 &= rang.contains(&t.2);
    println!("result => {}", t1);
}
*/