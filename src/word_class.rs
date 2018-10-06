
use std::fmt;
use std::str::FromStr;


#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum WordClassLevel {
    One,
    Two,
}

// GB/T 20532—2006: 信息处理用现代汉语词类标记规范 ( Standard of POS Tag of Contemporary Chinese for CIP )
/// 词类标记
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum WordClass {
    /// 形容词, adjective
    A,
    /// 性质形容词, adjective-quality
    AQ,
    /// 状态形容词, adjective-state
    AS,
    /// 连词, conjunction
    C,
    /// 副词, adverb
    D,
    /// 叹词, exclamation
    E,
    /// 区别词, difference
    F,
    /// 语素字, “根”的汉语拼音首字母
    G,
    /// 形容词性语素字, “根”的汉语拼音首字母-adjective
    GA,
    /// 名词性语素字, “根”的汉语拼音首字母-noun
    GN,
    /// 动词性语素字, “根”的汉语拼音首字母-verb
    GV,
    /// 前接成分, head
    H,
    /// 习用语, idiom
    I,
    /// 形容词性习用语, idiom-adjective
    IA,
    /// 连词性习用语, idiom-conjunction
    IC,
    /// 名词性习用语, idiom-noun
    IN,
    /// 动词性习用语, idiom-verb
    IV,
    /// 缩略语, “简”的汉语拼音首字母
    J,
    /// 形容词性缩略语, “简”的汉语拼音首字母-adjective
    JA,
    /// 名词性缩略语, “简”的汉语拼音首字母-noun
    JN,
    /// 动词性缩略语, “简”的汉语拼音首字母-verb
    JV,
    /// 后接成分, 依据通常做法
    K,
    /// 数词, numeral
    M,
    /// 名词, noun
    N,
    /// 方位名词, noun-direction
    ND,
    /// 普通名词, noun-general
    NG,
    /// 人名, noun-human
    NH,
    /// 机构名, noun-institution
    NI,
    /// 处所名词, noun-location
    NL,
    /// 族名, noun-nation
    NN,
    /// 地名, noun-space
    NS,
    /// 时间名词, noun-time
    NT,
    /// 其他专有名词, noun-“专”的汉语拼音首字母
    NZ,
    /// 拟声词, onomatopoeia
    O,
    /// 介词, preposition
    P,
    /// 量词, quantity
    Q,
    /// 代词, pronoun
    R,
    /// 助词, auxiliary
    U,
    /// 动词, verb
    V,
    /// 趋向动词, verb-direction
    VD,
    /// 不及物动词, verb-intransitive
    VI,
    /// 联系动词, verb-linking
    VL,
    /// 及物动词, verb-transitive
    VT,
    /// 能愿动词, verb-auxiliary
    VU,
    /// 其他, 依据通常做法
    W,
    /// 标点符号, 依据通常做法
    WP,
    /// 非汉字字符串, “w”-string
    WS,
    /// 其他未知符号, “w”-unknown
    WU,
    /// 非语素字, 依据通常做法
    X
}

impl WordClass {

    pub fn level(&self) -> WordClassLevel {
        use self::WordClass::*;

        match *self {
            A => WordClassLevel::One,
            AQ => WordClassLevel::Two,
            AS => WordClassLevel::Two,
            C => WordClassLevel::One,
            D => WordClassLevel::One,
            E => WordClassLevel::One,
            F => WordClassLevel::One,
            G => WordClassLevel::One,
            GA => WordClassLevel::Two,
            GN => WordClassLevel::Two,
            GV => WordClassLevel::Two,
            H => WordClassLevel::One,
            I => WordClassLevel::One,
            IA => WordClassLevel::Two,
            IC => WordClassLevel::Two,
            IN => WordClassLevel::Two,
            IV => WordClassLevel::Two,
            J => WordClassLevel::One,
            JA => WordClassLevel::Two,
            JN => WordClassLevel::Two,
            JV => WordClassLevel::Two,
            K => WordClassLevel::One,
            M => WordClassLevel::One,
            N => WordClassLevel::One,
            ND => WordClassLevel::Two,
            NG => WordClassLevel::Two,
            NH => WordClassLevel::Two,
            NI => WordClassLevel::Two,
            NL => WordClassLevel::Two,
            NN => WordClassLevel::Two,
            NS => WordClassLevel::Two,
            NT => WordClassLevel::Two,
            NZ => WordClassLevel::Two,
            O => WordClassLevel::One,
            P => WordClassLevel::One,
            Q => WordClassLevel::One,
            R => WordClassLevel::One,
            U => WordClassLevel::One,
            V => WordClassLevel::One,
            VD => WordClassLevel::Two,
            VI => WordClassLevel::Two,
            VL => WordClassLevel::Two,
            VT => WordClassLevel::Two,
            VU => WordClassLevel::Two,
            W => WordClassLevel::One,
            WP => WordClassLevel::Two,
            WS => WordClassLevel::Two,
            WU => WordClassLevel::Two,
            X => WordClassLevel::One,
        }
    }


    pub fn name(&self) -> &'static str {
        use self::WordClass::*;

        match *self {
            A => "形容词",
            AQ => "性质形容词",
            AS => "状态形容词",
            C => "连词",
            D => "副词",
            E => "叹词",
            F => "区别词",
            G => "语素字",
            GA => "形容词性语素字",
            GN => "名词性语素字",
            GV => "动词性语素字",
            H => "前接成分",
            I => "习用语",
            IA => "形容词性习用语",
            IC => "连词性习用语",
            IN => "名词性习用语",
            IV => "动词性习用语",
            J => "缩略语",
            JA => "形容词性缩略语",
            JN => "名词性缩略语",
            JV => "动词性缩略语",
            K => "后接成分",
            M => "数词",
            N => "名词",
            ND => "方位名词",
            NG => "普通名词",
            NH => "人名",
            NI => "机构名",
            NL => "处所名词",
            NN => "族名",
            NS => "地名",
            NT => "时间名词",
            NZ => "其他专有名词",
            O => "拟声词",
            P => "介词",
            Q => "量词",
            R => "代词",
            U => "助词",
            V => "动词",
            VD => "趋向动词",
            VI => "不及物动词",
            VL => "联系动词",
            VT => "及物动词",
            VU => "能愿动词",
            W => "其他",
            WP => "标点符号",
            WS => "非汉字字符串",
            WU => "其他未知符号",
            X => "非语素字",
        }
    }

}

impl fmt::Display for WordClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}


impl FromStr for WordClass {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::WordClass::*;

        match s {
            "A" | "a" => Ok(A),
            "AQ" | "aq" => Ok(AQ),
            "AS" | "as" => Ok(AS),
            "C" | "c" => Ok(C),
            "D" | "d" => Ok(D),
            "E" | "e" => Ok(E),
            "F" | "f" => Ok(F),
            "G" | "g" => Ok(G),
            "GA" | "ga" => Ok(GA),
            "GN" | "gn" => Ok(GN),
            "GV" | "gv" => Ok(GV),
            "H" | "h" => Ok(H),
            "I" | "i" => Ok(I),
            "IA" | "ia" => Ok(IA),
            "IC" | "ic" => Ok(IC),
            "IN" | "in" => Ok(IN),
            "IV" | "iv" => Ok(IV),
            "J" | "j" => Ok(J),
            "JA" | "ja" => Ok(JA),
            "JN" | "jn" => Ok(JN),
            "JV" | "jv" => Ok(JV),
            "K" | "k" => Ok(K),
            "M" | "m" => Ok(M),
            "N" | "n" => Ok(N),
            "ND" | "nd" => Ok(ND),
            "NG" | "ng" => Ok(NG),
            "NH" | "nh" => Ok(NH),
            "NI" | "ni" => Ok(NI),
            "NL" | "nl" => Ok(NL),
            "NN" | "nn" => Ok(NN),
            "NS" | "ns" => Ok(NS),
            "NT" | "nt" => Ok(NT),
            "NZ" | "nz" => Ok(NZ),
            "O" | "o" => Ok(O),
            "P" | "p" => Ok(P),
            "Q" | "q" => Ok(Q),
            "R" | "r" => Ok(R),
            "U" | "u" => Ok(U),
            "V" | "v" => Ok(V),
            "VD" | "vd" => Ok(VD),
            "VI" | "vi" => Ok(VI),
            "VL" | "vl" => Ok(VL),
            "VT" | "vt" => Ok(VT),
            "VU" | "vu" => Ok(VU),
            "W" | "w" => Ok(W),
            "WP" | "wp" => Ok(WP),
            "WS" | "ws" => Ok(WS),
            "WU" | "wu" => Ok(WU),
            "X" | "x" => Ok(X),
            _ => Err(()),
        }
    }
}
