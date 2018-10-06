use std::cmp;
use std::convert::TryFrom;

#[allow(non_snake_case)]
#[path = "./hmm_dict.rs"]
mod hmm_dict;
pub use self::hmm_dict::{ PROB_INIT, PROB_TRANS, PROB_EMIT_B, PROB_EMIT_E, PROB_EMIT_M, PROB_EMIT_S, PROB_EMIT };

pub type StatusSet = [f64; 4];
pub static MIN_FLOAT: f64 = -3.14e100;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Status {
    B,
    E,
    M,
    S,
}

impl Into<usize> for Status {
    fn into(self) -> usize {
        match self {
            Status::B => 0usize,
            Status::E => 1,
            Status::M => 2,
            Status::S => 3,
        }
    }
}
impl<'a> Into<usize> for &'a Status {
    fn into(self) -> usize {
        (*self).into()
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        match self {
            Status::B => 0u8,
            Status::E => 1,
            Status::M => 2,
            Status::S => 3,
        }
    }
}
impl<'a> Into<u8> for &'a Status {
    fn into(self) -> u8 {
        (*self).into()
    }
}

impl TryFrom<usize> for Status {
    type Error = ();
    fn try_from(n: usize) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Status::B),
            1 => Ok(Status::E),
            2 => Ok(Status::M),
            3 => Ok(Status::S),
            _ => Err(()),
        }
    }
}
impl<'a> TryFrom<&'a usize> for Status {
    type Error = ();
    fn try_from(n: &'a usize) -> Result<Self, Self::Error> {
        Status::try_from(*n)
    }
}

impl TryFrom<u8> for Status {
    type Error = ();
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Status::B),
            1 => Ok(Status::E),
            2 => Ok(Status::M),
            3 => Ok(Status::S),
            _ => Err(()),
        }
    }
}
impl<'a> TryFrom<&'a u8> for Status {
    type Error = ();
    fn try_from(n: &'a u8) -> Result<Self, Self::Error> {
        Status::try_from(*n)
    }
}


pub static PREV_STATUS: [[Status; 2]; 4] = [
    [Status::E, Status::S],  // B
    [Status::B, Status::M],  // E
    [Status::M, Status::B],  // M
    [Status::S, Status::E],  // S
];


pub fn viterbi(text: &str) -> Vec<Status> {
    let chars_len = text.chars().count();
    assert_eq!(chars_len > 0, true);
    
    // B, M, E, S
    let states = [Status::B, Status::M, Status::E, Status::S];

    #[allow(non_snake_case)]
    let mut V = vec![ [MIN_FLOAT; 4]; chars_len ];
    let mut path = vec![ vec![Status::B; chars_len]; 4];

    let get_emit_val = |y: &Status, c: &char| -> f64 {
        PROB_EMIT[*y as usize].binary_search_by_key(&c, |(k, _)| k )
                                    .map(|index| PROB_EMIT[*y as usize][index].1)
                                    .unwrap_or(MIN_FLOAT)
    };

    let mut i = 0usize;
    for c in text.chars() {
        if i == 0 {
            // init
            for y in states.iter() {
                let emit_val = get_emit_val(&y, &c);

                V[i][*y as usize] = PROB_INIT[*y as usize] + emit_val;
                path[*y as usize][i] = y.clone();
            }
        } else {
            let mut new_path = vec![vec![Status::B; chars_len]; 4];

            for y in states.iter() {
                let emit_val = get_emit_val(&y, &c);
                let (prob, state) = PREV_STATUS[*y as usize].iter()
                    .map(|y0| {
                        let n1 = V[i - 1][*y0 as usize];
                        let n2 = PROB_TRANS[*y0 as usize][*y as usize];
                        ( n1 + n2 + emit_val, *y0 )
                    })
                    .max_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal))
                    .unwrap();
                
                V[i][*y as usize] = prob;
                let mut prev_path = path[state as usize].clone();
                prev_path[i] = *y;
                new_path[*y as usize] = prev_path;
            }

            path = new_path;
        }

        i += 1;
    }

    let (_prob, state) = [Status::E, Status::S]
        .iter()
        .map(|y| (V[chars_len - 1][*y as usize], y))
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(cmp::Ordering::Equal))
        .unwrap();
    
    let best_path = path[*state as usize].clone();

    best_path
}


pub fn cut(sentence: &str) -> Vec<String> {
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    enum State {
        Han,
        Skip,
    }

    let mut words: Vec<String> = Vec::new();
    let mut state = State::Han;
    let mut block = String::new();

    let mut chars = sentence.chars();

    let work = |text: &str, words: &mut Vec<String>| {
        let pos_list = viterbi(&text);
        let mut word: Vec<char> = vec![];

        let mut i = 0usize;
        for c in text.chars() {
            let pos = pos_list[i];
            match pos {
                Status::B => {
                    word.clear();
                    word.push(c);
                },
                Status::M => {
                    word.push(c);
                },
                Status::E => {
                    word.push(c);
                    words.push(word.iter().collect());
                    word.clear();
                },
                Status::S => {
                    words.push( c.to_string() );
                    word.clear();
                },
            }

            i += 1;
        }
    };

    loop {
        match chars.next() {
            Some(c) => {
                let code = c as u32;
                if c.is_control() || c.is_whitespace() || c.is_ascii_punctuation() {
                    continue;
                } else if code >= 0x4E00 && code <= 0x9FD5 {
                    if state != State::Han {
                        if block.len() > 0 {
                            words.push(block.clone());
                            block.clear();
                        }
                        state = State::Han;
                    }
                    block.push(c);
                } else {
                    if state == State::Han {
                        if block.len() > 0 {
                            // Cut
                            work(&block, &mut words);
                            block.clear();
                        }
                        state = State::Skip;
                    }
                    block.push(c);
                }
            },
            None => {
                if block.len() > 0 {
                    if state != State::Han {
                        words.push(block.clone());
                    } else {
                        // Cut
                        work(&block, &mut words);
                    }
                    block.clear();
                }
                break;
            },
        }
    }

    words
}


#[cfg(test)]
mod test {
    
    use super::{ Status, viterbi, cut, };

    #[test]
    fn test_viterbi() {
        use self::Status::*;

        let sentence = "小明硕士毕业于中国科学院计算所";
        let path = viterbi(sentence);
        assert_eq!(path, vec![B, E, B, E, B, M, E, B, E, B, M, E, B, E, S]);
    }

    #[test]
    fn test_hmm_cut() {
        let sentence = "小明硕士毕业于中国科学院计算所";
        let words = cut(sentence);
        assert_eq!(words, vec!["小明", "硕士", "毕业于", "中国", "科学院", "计算", "所"]);
    }
}
