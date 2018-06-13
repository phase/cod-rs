use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    for arg in args {
        let file = arg.as_str();
        let f = BufReader::new(File::open(file).expect("open failed"));

        // each file interacts with its own register
        let mut register: i32 = 0;

        for line in f.lines() {
            for c in line.expect(&format!("line reading failed {} in", file)).chars() {
                register = execute(c, register);
            }
        }
    }

}

fn execute(c: char, reg: i32) -> i32 {
    let mut o = reg;
    if o == 256 || o == 1 { 
        o = 0;
    }

    match c {
        'i' | 'x' => o += 1,
        'd' => o -= 1,
        'e' => o = 0,
        'o' | 'c' => print!("{}", (o as u8) as char),
        's' | 'k' => o *= o,
        'q' => o = (o as f64).sqrt() as i32,
        'b' => o = !o,
        '&' => unsafe { o = *(o as *const i32) }
        'r' => o >>= 1,
        'l' => o <<= 1,
        'n' => print!("{}", o),
        '4' => {
            for i in (o..0).rev() {
                print!("{}", i);
            }
            o = 0;
        }
        '5' => {
            for i in (o..0).rev() {
                print!("{}", i);
            }
        }
        '6' => {
            let s = o * o;
            for i in o..s {
                print!("{}", i);
            }
            o = s;
        }
        _ => {}
    }

    o
}
