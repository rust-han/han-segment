extern crate han_segment;


use han_segment::hmm;

use std::time;
use std::env;


#[allow(dead_code)]
fn cut50() {
    let text = include_str!("../data/weicheng.txt");
    let now = time::Instant::now();
    
    for _i in 0..50 {
        let _words = hmm::cut(text);
    }
    
    let elapsed = now.elapsed();
    
    let milliseconds = elapsed.as_secs() * 1_000 + (elapsed.subsec_nanos() / 1_000_000) as u64;
    println!("elapsed: {:?} ms", milliseconds);
}

fn main() {
    // let text = "小明硕士毕业于中国科学院计算所";
    // let mut text = String::new();
    // io::stdin().read_to_string(&mut text).unwrap();
    
    let text = env::args().nth(1).unwrap();

    let now = time::Instant::now();
    let words = hmm::cut(&text);
    let elapsed = now.elapsed();

    println!("text: {:?}", text);
    println!("words: {:?}", words);

    let milliseconds = elapsed.as_secs() * 1_000 + (elapsed.subsec_nanos() / 1_000_000) as u64;
    println!("elapsed: {:?} ms", milliseconds);
}
