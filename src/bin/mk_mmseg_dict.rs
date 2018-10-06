use std::collections::HashSet;

fn main() {
    let words_dict = include_str!("../../data/mmseg/words.dic");
    let chars_dict = include_str!("../../data/mmseg/chars.dic");
    let _units_dict = include_str!("../../data/mmseg/units.dic");
    
    let mut words = words_dict.split_whitespace()
        .map(|line| line.trim().to_string() )
        .filter(|line| line.len() > 0 )
        .collect::<HashSet<String>>()
        .iter()
        .cloned()
        .collect::<Vec<String>>();

    words.sort();
    let max_word_length = words.iter().map(|word| word.chars().count()).max().unwrap();


    let mut chars = chars_dict.split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let tmp = line.split(' ').collect::<Vec<&str>>();

            let c = tmp[0].chars().nth(0).unwrap();
            let freq = tmp[1].parse::<usize>().unwrap();
            (c, freq)
        })
        .collect::<HashSet<(char, usize)>>()
        .iter()
        .cloned()
        .collect::<Vec<(char, usize)>>();
    chars.sort_by_key(|&(c, _freq)| c);

    let code = format!("
pub const MAX_WORD_LENGTH: usize = {};
pub static WORDS_DICT: [&str; {}] = {:?};
pub static CHARS_DICT: [(char, usize); {}] = {:?};

", max_word_length,
    words.len(),
    words,
    chars.len(),
    chars,
    );

    println!("{}", code);
}