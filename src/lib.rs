pub mod generation;
pub mod output;
pub mod stats;

use ahash::AHashMap;
use clap::Parser;
use std::fs;

#[derive(PartialEq, Debug, Clone)]
pub struct Key {
    pub hand: u8,
    pub finger: Finger,
    pub row: u8,
    pub lateral: bool,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "whirl.txt")]
    pub layout: String,

    #[arg(short, long, default_value = "mr.txt")]
    pub corpus: String,

    #[arg(default_value = "analyze")]
    pub command: String,

    #[arg(short, long, default_value_t = 500)]
    pub iterations: u64,

    #[arg(short, long, default_value_t = 10)]
    pub magic_rules: usize,

    #[arg(long, default_value_t = 0.99)]
    pub cooling: f64,

    #[arg(long, action)]
    pub compact: bool,
/*
    #[arg(short, long)]
    pub algorithm: Algorithm, */
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Default, Clone, Debug)]
pub struct Layout {
    pub layout: [char; 32],
    pub stats: Stats,
    pub magic: AHashMap<char, char>,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub enum Algorithm {
    #[default]
    SimAnnealing,
    RandomLayout,
    GreedySwapping,
    HillClimbing,
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
    hsb: i64,
    hss: i64,
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

pub fn load_magic_rules(layout: &String) -> AHashMap<char, char> {
    let layout_letters = load_layout_letters(layout);
    let magic_rules_raw = layout_letters[36..].split('\n').filter(|s| !s.is_empty());
    let mut magic_rules: AHashMap<char, char> = AHashMap::default();

    for rule in magic_rules_raw {
        magic_rules.insert(
            rule.chars().next().expect("rule is empty"),
            rule.chars().nth(1).expect("rule has only 1 character"),
        );
    }
    magic_rules
}

pub fn load_layout(layout: &String) -> [char; 32] {
    let layout_letters = load_layout_letters(layout);
    // has to be 37 because ⎵ is a few extra bytes
    layout_letters[..35]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout")
}

fn load_layout_letters(layout: &String) -> String {
    let layout_letters: String = fs::read_to_string("layouts/".to_owned() + &layout)
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .chars()
        .collect();
    layout_letters
}
