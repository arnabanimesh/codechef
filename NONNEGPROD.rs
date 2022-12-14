use std::fmt::Write;
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let mut vec = input.lock().lines();
    vec.next();
    let mut buf = String::with_capacity(4096);
    loop {
        vec.next();
        match vec.next() {
            Some(i) => {
                let x = i.unwrap();
                if x.contains(" 0") || x.starts_with("0") {
                    writeln!(buf,"0").unwrap();
                    continue;
                }
                match x.matches('-').count() & 1 {
                    1 => {writeln!(buf,"1").unwrap();}
                    _ => {writeln!(buf,"0").unwrap();}
                }
            }
            None => {
                break;
            }
        }
    }
    print!("{}", buf);
}
