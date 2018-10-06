#![feature(test)]

extern crate test;
extern crate han_segment;


use han_segment::hmm;
use han_segment::mmseg::WordSegmentation;


#[bench]
fn bench_hmm_weicheng(b: &mut test::Bencher) {
    let text = include_str!("../data/weicheng.txt");
    b.iter(|| {
        let _ = hmm::cut(text);
    });
    b.bytes += text.len() as u64;
}

#[bench]
fn bench_mmseg_weicheng(b: &mut test::Bencher) {
    let text = include_str!("../data/weicheng.txt");
    b.iter(|| {
        let _ = text.words();
    });
    b.bytes += text.len() as u64;
}