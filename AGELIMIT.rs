use std::cmp::Ordering;
use std::fmt::Write;
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let mut vec = input.lock().lines();
    vec.next();
    let mut buf = String::with_capacity(4096);
    loop {
        match vec.next() {
            Some(i) => {
                let x: Vec<u8> = i
                    .unwrap()
                    .trim()
                    .split_ascii_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect();
                match x[0].cmp(&x[2]) {
                    Ordering::Greater => {
                        writeln!(buf, "NO").unwrap();
                    },
                    _ => match x[1].cmp(&x[2]) {
                        Ordering::Greater => {
                            writeln!(buf, "YES").unwrap();
                        }
                        _ => {
                            writeln!(buf, "NO").unwrap();
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    print!("{}", buf);
}
