extern crate han_segment;

use han_segment::mmseg::{ WordSegmentation, RED_CHAR };


fn main() {
    let text = "或许这种现象可以勉强归因于“小孩子不懂事”，但种族歧视的现象却不会伴随着人的长大而消失。
相反，年龄大的人的歧视行为可能更难以让人接受。
马塔对我说：“还有一次，我在南京的一个超市里，遇到一个可能有五六十岁的人，过来之后就想摸一下我的皮肤。
当时我刚到中国，我就特别的生气。”";

    println!("{}", text);
    println!("{}{}{}", RED_CHAR, text.words().join(RED_CHAR), RED_CHAR);
}