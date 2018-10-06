use std::collections::HashSet;


#[derive(Debug, PartialEq, Eq)]
enum State {
    Pending,
    Start,
    Trans,
    Emit,
}

fn main(){
    let data = include_str!("../../data/hmm_model.txt");
    
    let mut lines = data.lines();
    let mut state = State::Pending;

    let mut prob_start = [0f64; 4];
    let mut prob_trans = [ [0f64; 4]; 4];
    let mut prob_emit: [ Vec<(char, f64)>; 4 ]  = [ vec![], vec![], vec![], vec![] ];

    loop {
        if state == State::Emit {
            break;
        }

        let line = lines.next().unwrap().trim();

        if line.starts_with("#prob_start") {
            assert_eq!(state, State::Pending);

            let value_line = lines.next().unwrap().trim();
            let values = value_line.split(" ").map(|s| s.parse::<f64>().unwrap() ).collect::<Vec<f64>>();
            
            assert_eq!(values.len(), 4);

            prob_start[0] = values[0];
            prob_start[1] = values[1];
            prob_start[2] = values[2];
            prob_start[3] = values[3];

            state = State::Start;

        } else if line.starts_with("#prob_trans") {
            assert_eq!(state, State::Start);

            for y in 0..prob_trans.len() {
                let value_line = lines.next().unwrap().trim();
                let values = value_line.split(" ").map(|s| s.parse::<f64>().unwrap() ).collect::<Vec<f64>>();
                
                assert_eq!(values.len(), 4);

                prob_trans[y][0] = values[0];
                prob_trans[y][1] = values[1];
                prob_trans[y][2] = values[2];
                prob_trans[y][3] = values[3];
            }

            state = State::Trans;

        } else if line.starts_with("#prob_emit") {
            assert_eq!(state, State::Trans);

            let mut i = 0;
            for kind in ["#B", "#E", "#M", "#S"].iter() {
                let line = lines.next().unwrap().trim();
                assert_eq!(line, *kind);

                let line = lines.next().unwrap().trim();
                let mut map = line.split(",").map(|kv| {
                    let pair = kv.split(":").collect::<Vec<&str>>();
                    assert_eq!(pair.len(), 2);
                    let key = pair[0].trim().chars().collect::<Vec<char>>()[0];
                    let val = pair[1].trim().parse::<f64>().unwrap();

                    (key, val)
                }).collect::<Vec<(char, f64)>>();

                let map_keys = map.iter().map(|(k, _v)| *k).collect::<HashSet<char>>();
                assert_eq!(map.len(), map_keys.len());
                
                // map.sort_unstable_by(|a, b| (a.0 as usize).cmp(&(b.0 as usize)) );
                map.sort_unstable_by_key(| &(k, _)| k );
                
                prob_emit[i] = map;

                i += 1;
            }

            state = State::Emit;
        }
    }

    let code = format!("
pub static PROB_INIT: [f64; 4]= {:?};
pub static PROB_TRANS: [ [f64; 4]; 4 ] = {:?};
pub static PROB_EMIT_B: [(char, f64); {}] = {:?};
pub static PROB_EMIT_E: [(char, f64); {}] = {:?};
pub static PROB_EMIT_M: [(char, f64); {}] = {:?};
pub static PROB_EMIT_S: [(char, f64); {}] = {:?};
pub static PROB_EMIT: [&[(char, f64)]; 4] = [ &PROB_EMIT_B, &PROB_EMIT_E, &PROB_EMIT_M, &PROB_EMIT_S];
    ", prob_start,
    prob_trans,
    prob_emit[0].len(), prob_emit[0],
    prob_emit[1].len(), prob_emit[1],
    prob_emit[2].len(), prob_emit[2],
    prob_emit[3].len(), prob_emit[3],
    );

    println!("{}", code);
}