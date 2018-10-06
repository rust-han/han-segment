

#[allow(non_snake_case)]
#[path = "./mmseg_dict.rs"]
mod mmseg_dict;
pub use self::mmseg_dict::{ WORDS_DICT, MAX_WORD_LENGTH, CHARS_DICT, };

pub const RED_CHAR: &str = "\x1b[31m|\x1b[0m";


#[derive(Debug)]
pub enum Method {
    Simple,
    Complex,
}


fn is_word(chars: &[char]) -> bool {
    if chars.len() == 0 {
        return false;
    }

    if chars.len() == 1 {
        return true;
    }

    let s = chars.iter().collect::<String>();
    WORDS_DICT.binary_search(&s.as_ref())
        .map(|_| true)
        .unwrap_or(false)
}


fn f1(chars: &[char], start: usize) -> Vec<(usize, usize)> {
    let mut end: usize   = start + 1;
    let mut words: Vec<(usize, usize)> = vec![];

    loop {
        if start >= chars.len() {
            break;
        }

        if (end - start) > MAX_WORD_LENGTH || end > chars.len() {
            break;
        }

        if is_word(&chars[start..end]) {
            words.push((start, end));
            end += 1;
        } else {
            break;
        }
    }

    return words;
}

fn f2(chars: &[char], groups: &Vec<Vec<(usize, usize)>>) -> (usize, usize){
    assert_eq!(groups.len() > 0, true);
    
    if groups.len() == 1 {
        debug!("规则0: 通过 ...");
        return groups[0][0];
    }

    // 规则一: 选择词组长度最大的那个词组
    let max_len = groups.iter().map(|group| group.len()).max().unwrap();
    let groups = groups.iter()
                    .filter(|group| group.len() == max_len )
                    .cloned()
                    .collect::<Vec<Vec<(usize, usize)>>>();

    if groups.len() == 1 {
        debug!("规则1: 通过 ...");
        return groups[0][0];
    }

    // 规则二: 选择平均词语长度最大的那个（平均词长＝词组总字数／词语数量）
    let avg_word_length = groups.iter()
        .map(|group| {
            let total_chars_num: f64 = group.iter()
                .map(|&(start, end)| end - start)
                .sum::<usize>() as f64;
            let total_word_num: f64 = group.len() as f64;
            total_chars_num / total_word_num
        })
        .collect::<Vec<f64>>();

    use std::f64;
    let max_len = avg_word_length.iter()
        .fold(f64::MIN, |acc, x| {
            if !x.is_finite() { panic!("Ooops ...") }
            acc.max(*x)
        });
    let groups = groups.iter()
                    .zip(avg_word_length.iter())
                    .filter(|&(_, avg_len)| avg_len == &max_len )
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<Vec<(usize, usize)>>>();

    if groups.len() == 1 {
        debug!("规则2: 通过 ...");
        return groups[0][0];
    }

    // 规则三: 选择词长变化最小的那个
    let changes = groups.iter()
        .zip(avg_word_length.iter())
        .map(|(group, avg_len)| {
            // 研究生_命_起源 标准差=sqrt( ( (2-3)^2 + (2-1)^2 + (2-2)^2 )/3) = 0.8165
            let n = group.iter().map(|&(start, end)| {
                let len = (end - start) as f64;
                let d = avg_len - len;
                // BUG: Rust 语言负数不能 参与 powf 运算。
                if d < 0.0 {
                    warn!("Rust 语言负数不能 参与 `f64.powf(f32)` 运算");
                    0.0f64
                } else {
                    d.powf(*avg_len)
                }
            }).sum::<f64>();
            
            (n / (group.len() as f64)).sqrt()
        })
        .collect::<Vec<f64>>();
    
    let min_len = changes.iter()
        .fold(f64::MAX, |acc, x| {
            if !x.is_finite() { panic!("Ooops ...") }
            acc.min(*x)
        });
    
    let groups = groups.iter()
                    .zip(changes.iter())
                    .filter(|&(_, change)| change == &min_len )
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<Vec<(usize, usize)>>>();

    if groups.len() == 1 {
        debug!("规则3: 通过 ...");
        return groups[0][0];
    }

    // 规则四: 选择单字词的出现频率统计值最高的那组
    let char_freqs = groups.iter()
            .map(|group|{
                let freqs = group.iter().filter(|&(start, end)| end - start == 1)
                    .map(|&(start, _)| {
                        let c = chars[start];
                        CHARS_DICT.binary_search_by(|&(cc, _)| cc.cmp(&c))
                            .map(|index| CHARS_DICT[index].1 as f64)
                            .unwrap_or(0.0f64)
                    })
                    .collect::<Vec<f64>>();
                
                assert_eq!(freqs.len(), 2);

                freqs[0].log(freqs[1])
            })
            .collect::<Vec<f64>>();

    assert_eq!(char_freqs.len() > 0, true);

    let max_freq = char_freqs.iter()
        .fold(f64::MIN, |acc, x| {
            if !x.is_finite() { panic!("Ooops ...") }
            acc.max(*x)
        });

    let groups = groups.iter()
                    .zip(char_freqs.iter())
                    .filter(|&(_, freq)| freq == &max_freq)
                    .map(|(group, _)| group.clone())
                    .collect::<Vec<Vec<(usize, usize)>>>();

    assert_eq!(groups.len() > 0, true);

    if groups.len() == 1 {
        debug!("规则4: 通过 ...");
        return groups[0][0];
    } else {
        warn!("规则4: 剩下更多的可能无法判断，返回第一项！");
        return groups[0][0];
    }
}

pub fn simple_cut(chars: &[char], output: &mut Vec<String>) {
    assert_eq!(chars.len() > 0, true);

    let mut start: usize = 0;

    loop {
        if start >= chars.len() {
            break;
        }

        let groups = f1(&chars, start);

        if groups.len() == 0 {
            error!("在 {} 处意外终止！", start);
            break;
        }

        let best_word = if groups.len() == 1 {
            groups[0]
        } else {
            // 规则一: 选择长度最大的那个词汇
            let max_len = groups.iter()
                            .map(|&(start, end)| end - start)
                            .max()
                            .unwrap();
            let groups = groups.iter()
                    .filter(|&(start, end)| (end - start) == max_len )
                    .cloned()
                    .collect::<Vec<(usize, usize)>>();

            assert_eq!(groups.len() > 0, true);

            groups[0]
        };

        start = best_word.1;

        {
            let (start, end) = best_word;
            output.push(chars[start..end].iter().collect::<String>());
        }
    }
}

fn complex_cut(chars: &[char], output: &mut Vec<String>) {
    assert_eq!(chars.len() > 0, true);

    let mut start: usize = 0;

    loop {
        if start >= chars.len() {
            break;
        }

        let groups = f1(&chars, start).iter()
            .map(|&(start, end)| {
                let mut res=  f1(&chars, end);
                res.insert(0, (start, end));
                res
            })
            .collect::<Vec<Vec<(usize, usize)>>>();
        
        if groups.len() == 0 {
            error!("在 {} 处意外终止！", start);
            break;
        }

        let best_word = f2(&chars, &groups);

        start = best_word.1;

        {
            let (start, end) = best_word;
            output.push(chars[start..end].iter().collect::<String>());
        }
        
    }
}



pub trait WordSegmentation {
    fn words(&self) -> Vec<String>;
}

impl WordSegmentation for [char] {
    fn words(&self) -> Vec<String> {
        (&self).words()
    }
}

impl<'a> WordSegmentation for &'a [char] {
    fn words(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        complex_cut(&self, &mut output);
        output
    }
}

impl<'a> WordSegmentation for &'a str {
    fn words(&self) -> Vec<String> {
        self.chars().collect::<Vec<char>>().words()
    }
}


