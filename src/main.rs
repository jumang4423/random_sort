use rand::seq::SliceRandom;
use std::fs;
use std::str::FromStr;
use std::vec::*;
use stopwatch;
fn main() {
    let mut stop_watch = stopwatch::Stopwatch::new();

    println!("loading ./input_data...");

    let buf = fs::read_to_string("input_data").unwrap();
    let mv = buf.split_whitespace().map(|n| i32::from_str(n).unwrap());
    let mut vec: Vec<i32> = Vec::new();

    for d in mv {
        vec.push(d);
    }

    println!("file loaded. calculating...");

    stop_watch.start();

    loop {
        let new_vec: Vec<i32> = vec
            .choose_multiple(&mut rand::thread_rng(), vec.len())
            .cloned()
            .collect();

        if check_sorted(&new_vec) {
            stop_watch.stop();
            println!("result is: {:?}", new_vec);
            println!("elapsed: {}ms", stop_watch.elapsed_ms());
        }

        println!("x: {:?}", new_vec);
    }
    println!("finished");
}

pub fn check_sorted(vec: &Vec<i32>) -> bool {
    let mut min = i32::MIN;

    for d in vec {
        if min <= *d {
            min = d.clone();
        } else {
            return false;
        }
    }

    return true;
}
