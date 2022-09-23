use std::fmt::Write;
use std::io::{stdin, BufRead};

fn main() {
    let mut vec = stdin().lock().lines();
    let t: usize = vec
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse t as int");
    let mut buf = String::with_capacity(4096);
    for _ in 0..t {
        match vec.next() {
            Some(i) => {
                let n: usize = i.unwrap().trim().parse().expect("Failed to parse n as int");
                let mut s = vec.next().unwrap().unwrap().trim().as_bytes().to_owned();
                s.sort_unstable();
                let (mut idxs, mut addc, mut subc) = (0, 0, 0);
                while s[idxs] < 48 {
                    match s[idxs] {
                        43 => addc += 1,
                        _ => subc += 1,
                    }
                    idxs += 1;
                }
                let s = s[idxs..].to_owned();
                let mut ans: Vec<u8> = vec![0; n];
                let mut idxa = n - 1;
                let mut idxs = 0;
                while subc > 0 {
                    ans[idxa] = s[idxs];
                    ans[idxa - 1] = 45;
                    idxa -= 2;
                    idxs += 1;
                    subc -= 1;
                }
                while addc > 0 {
                    ans[idxa] = s[idxs];
                    ans[idxa - 1] = 43;
                    idxa -= 2;
                    idxs += 1;
                    addc -= 1;
                }
                for i in idxs..s.len() {
                    ans[idxa] = s[i];
                    idxa -= 1;
                }
                writeln!(buf, "{}", String::from_utf8(ans).unwrap()).unwrap();
            }
            None => {
                break;
            }
        }
    }
    print!("{}", buf);
}
