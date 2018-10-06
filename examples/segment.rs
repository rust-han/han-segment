#![allow(unused_imports, unused_variables, dead_code)]

extern crate han_segment;
extern crate unic_segment;
extern crate unic_ucd_common;
extern crate unic_ucd_segment;

use unic_segment::{ GraphemeIndices, Graphemes, WordBoundIndices, WordBounds, Words };
use unic_ucd_common::is_alphanumeric;
use unic_ucd_segment::{ GraphemeClusterBreak, SentenceBreak, WordBreak, };


fn main() {
    let text = "
在美国总统唐纳德•特朗普(Donald Trump)威胁要对中国对美出口全部产品征收惩罚性关税之际，
中国政府邀请华尔街顶级银行家参加在北京仓促安排的一次会议。
据1988位知情人士透露，中共官员邀请美国主要金融机构的负责人参加9月16日在北京召开的
“中美金融圆桌会议”(China-US Financial Round­table)，
之后会晤中国国家副主席王岐山。";
    
    println!("\n样本: \n{}", &text);


    println!("\n最小文字单元: \n{:?}", Graphemes::new(&text).collect::<Vec<&str>>());

    println!("\nUnicode分词: \n{:?}", 
        Words::new(&text, |s: &&str| s.chars().any(is_alphanumeric),).collect::<Vec<&str>>()
    );


    let ss = text.replace("\r\n", "\n").replace("\n", "");
    

    let temp = text.chars()
        .map(|c| ( c, SentenceBreak::of(c) ) )
        .collect::<Vec<(char, SentenceBreak)>>();
    

    println!("\n字符类别: \n{:?}", temp);
    
    let mut output = String::new();
    let mut ignore_term = false;
    for &(c, sb) in &temp {
        output.push(c);
        let mut b = false;

        if sb == SentenceBreak::Close {
            if ignore_term == true {
                b = true;
            }
            ignore_term = !ignore_term;
        }

        if sb == SentenceBreak::STerm || sb == SentenceBreak::LF {
            b = true;
        }

        if ignore_term == false && b == true {
            output.push_str("\x1b[31m|\x1b[0m");
        }
    }

    println!("\n断句: \n{}", output);
}