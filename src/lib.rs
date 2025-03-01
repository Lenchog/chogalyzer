pub mod generation;
pub mod output;
pub mod stats;

use ahash::AHashMap;

#[derive(PartialEq)]
pub struct Key {
    hand: u8,
    finger: Finger,
    row: u8,
    lateral: bool,
}

#[derive(Eq, Hash, PartialEq, PartialOrd)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    score: f64,
    fspeed: i64,
    sfb: i64,
    sfr: i64,
    sfs: i64,
    lsb: i64,
    lss: i64,
    fsb: i64,
    fss: i64,
    inroll: i64,
    outroll: i64,
    alt: i64,
    inthreeroll: i64,
    outthreeroll: i64,
    weak_red: i64,
    red: i64,
    heatmap: i64,
    thumb_stat: u32,
    pub bigrams: u32,
    pub skipgrams: u32,
    pub trigrams: u32,
    pub ngram_table: AHashMap<[char; 3], u32>,
    pub bad_bigrams: AHashMap<[char; 2], u32>,
}

const INCLUDE_THUMB_ALT: bool = true;
const INCLUDE_THUMB_ROLL: bool = true;
