pub mod generation;
pub mod output;
pub mod stats;

use ahash::AHashMap;
use clap::Parser;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::stats::layout_raw_to_table;

/// Contains all information about a key's position
#[derive(PartialEq, Debug, Clone)]
pub struct Key {
    pub hand: u8,
    pub finger: Finger,
    pub row: u8,
    pub lateral: bool,
}

/// Args that can be used
#[derive(Parser, Debug)]
pub struct Args {
    /// Which layout to use
    #[arg(short, long, default_value = "whirl.txt")]
    pub layout: String,

    /// Which corpus to use
    #[arg(short, long, default_value = "mr.txt")]
    pub corpus: String,

    /// Which command to use TODO find other stuff
    #[arg(default_value = "analyze")]
    pub command: String,

    /// For generation. How many swaps the analyser will do
    #[arg(short, long, default_value_t = 500)]
    pub iterations: u64,

    /// For generation. How many magic rules will be generated
    #[arg(short, long, default_value_t = 10)]
    pub magic_rules: usize,

    /// For sim-annealing, the cooling rate
    #[arg(long, default_value_t = 0.99)]
    pub cooling: f64,

    /// Whether to use compact formatting
    #[arg(long, action)]
    pub compact: bool,
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

/// General use struct for all layouts
#[derive(Default, Clone, Debug)]
pub struct Layout {
    /// The actual letters in the layout
    pub layout: [char; 32],
    /// The magic rules attached
    pub magic: AHashMap<char, char>,
    /// The analysed stats. Maybe will be changed in the future
    pub stats: Stats,
}

/// Contains information for which algorithm to use
#[derive(Default, PartialEq, Clone, Debug)]
pub enum Algorithm {
    /// The best algorithm. Uses weird heat stuff. Look it up
    #[default]
    SimAnnealing,
    /// The worst algorithm. Generates random algorithms
    RandomLayout,
    /// Naive option. Always chooses better layout
    GreedySwapping,
    /// Always takes the best swap. Oxelyzer uses this
    HillClimbing,
    /// Experimental, probably don't use
    Hybrid,
}
impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Algorithm::SimAnnealing => write!(f, "SimAnnealing"),
            Algorithm::RandomLayout => write!(f, "RandomLayout"),
            Algorithm::GreedySwapping => write!(f, "GreedySwapping"),
            Algorithm::HillClimbing => write!(f, "HillClimbing"),
            Algorithm::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// Struct to hold every stat, attached to layouts and used to keep track
#[derive(Default, Debug, Clone)]
pub struct Stats {
    /// Overall score
    score: f64,
    /// Weighted score of all same finger stats
    fspeed: i64,
    /// Same Finger Bigram
    /// e.g. `ed` on Qwerty
    sfb: i64,
    /// Same Finger Repeat
    /// e.g. `ll` on Qwerty
    sfr: i64,
    /// Same Finger Skipgram
    /// e.g. `e_d` on Qwerty
    sfs: i64,
    /// Lateral Stretch Bigram
    /// e.g. `dg` on Qwerty
    lsb: i64,
    /// Lateral Stretch Skipgram
    /// e.g. `d_g` on Qwerty
    lss: i64,
    /// Full Scissor Bigram
    /// e.g. `mi` on Qwerty
    fsb: i64,
    /// Full Scissor Skipgram
    /// e.g. `m_i` on Qwerty
    fss: i64,
    /// Half Scissor Bigram
    /// e.g. `ku` on Qwerty
    hsb: i64,
    /// Half Scissor Skipgram
    /// e.g. `k_u` on Qwerty
    hss: i64,
    /// Two keys on one hand rolling inwards, one key on the other hand
    /// e.g. `ask` on Qwerty
    inroll: i64,
    /// Two keys on one hand rolling outwards, one key on the other hand
    /// e.g. `thi` on Qwerty
    outroll: i64,
    /// Alternation
    /// e.g. `the` on Qwerty
    alt: i64,
    /// Three keys on one hand rolling inwards
    /// e.g. `wer` on Qwerty
    inthreeroll: i64,
    /// Three keys on one hand rolling outwards
    /// e.g. `rea` on Qwerty
    outthreeroll: i64,
    /// Keys redirecting
    /// e.g. `you` on Qwerty
    red: i64,
    /// Keys redirecting, but not without a thumb or index
    /// e.g. `was` on Qwerty
    weak_red: i64,
    /// Weighted penalty of usage of each key
    heatmap: i64,
    /// Penalty for utilising each column. To ensure pinky is not too heavy
    column_pen: i64,
    /// How many ngrams include thumb
    thumb_stat: u32,
    /// Total count of skipgrams
    /// (this may be equal to chars)
    pub skipgrams: u32,
    /// Total count of characters
    pub chars: u32,
    /// Table of how often each ngram occurs
    pub ngram_table: AHashMap<[char; 3], u32>,
    /// Table of how often each bad bigram occurs
    /// SFB, FSB, HSB, LSB
    pub bad_bigrams: AHashMap<[char; 2], u32>,
}

const INCLUDE_THUMB_ALT: bool = true;
const INCLUDE_THUMB_ROLL: bool = true;

/// Get hashmap of magic rules from layout name
pub fn load_magic_rules(layout: &str) -> AHashMap<char, char> {
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

/// Get array of layout letters from layout name
pub fn load_layout(layout: &str) -> [char; 32] {
    let layout_letters = load_layout_letters(layout);
    // has to be 37 because ⎵ is a few extra bytes
    layout_letters[..35]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout")
}

/// get a string of the layout from the layout name
fn load_layout_letters(layout: &str) -> String {
    let layout_letters: String = fs::read_to_string("layouts/".to_owned() + layout)
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .chars()
        .collect();
    layout_letters
}

pub fn load_three_keys(letters: &str) -> (Key, Key, Key) {
    let layout_raw = &load_layout("whirl.txt");
    let table = layout_raw_to_table(layout_raw);
    let key1 = table[&letters.chars().next().unwrap()].clone();
    let key2 = table[&letters.chars().nth(1).unwrap()].clone();
    let key3 = table[&letters.chars().nth(2).unwrap()].clone();
    (key1, key2, key3)
}

pub fn load_two_keys(letters: &str) -> (Key, Key) {
    let layout_raw = &load_layout("whirl.txt");
    let table = layout_raw_to_table(layout_raw);
    let key1 = table[&letters.chars().next().unwrap()].clone();
    let key2 = table[&letters.chars().nth(1).unwrap()].clone();
    (key1, key2)
}

/// Filter corpus with only letters from the layout and processes magic rules
fn filter_corpus(corpus_name: &str, layout_raw: &[char; 32]) -> String {
    println!("{}", "corpora/raw/".to_owned() + corpus_name);
    let corpus: String = fs::read_to_string("corpora/raw/".to_owned() + corpus_name)
        .expect("error reading corpus")
        .replace("\n\n", "")
        .replace(' ', "_")
        .chars()
        .flat_map(|ch| {
            if ch.is_ascii_uppercase() {
                // Replace uppercase letters with "*" followed by lowercase
                format!("*{}", ch.to_ascii_lowercase())
                    .chars()
                    .collect::<Vec<_>>()
            } else {
                vec![ch]
            }
        })
        .filter(|ch| layout_raw.contains(ch))
        .collect();
    let mut write_file =
        File::create("corpora/filtered/".to_owned() + corpus_name).expect("couldn't write corpus");
    let _ = write_file.write_all(corpus.as_bytes());
    corpus.to_string()
}

/// Load corpus from corpus name, and filter it with the layout name if it has not been previously filtered
pub fn load_corpus(corpus_name: &str, layout_name: &str) -> String {
    let layout = load_layout(layout_name);
    match fs::read_to_string("corpora/filtered/".to_owned() + corpus_name) {
        Ok(corpus) => corpus,
        Err(_) => {
            println!("couldn't find corpus, now loading");
            filter_corpus(corpus_name, &layout)
        }
    }
}

/// Standalone function that converts functions from Whirl to something else so I can try it out
// TODO make it work for other layouts
pub fn convert_corpus(new_layout_name: &str, corpus_name: &str) {
    let old_layout_name: &String = &String::from("whirl.txt");
    let old_layout = load_layout(old_layout_name);
    let new_layout = load_layout(new_layout_name);
    let old_magic_rules = load_magic_rules(old_layout_name);
    let new_magic_rules = load_magic_rules(new_layout_name);
    let mut corpus = load_corpus(corpus_name, old_layout_name);

    for letter in new_layout {
        let rule: [char; 2] = match new_magic_rules.get(&letter) {
            Some(other_letter) => [letter, *other_letter],
            None => [letter, letter],
        };
        corpus = corpus.replace(&rule.iter().collect::<String>(), &format!("{letter}*"));
    }

    let hash = new_layout
        .iter()
        .zip(old_layout)
        .collect::<AHashMap<_, _>>();

    let mut new_corpus: String = corpus
        .chars()
        .map(|c| hash.get(&c).copied().unwrap_or(c))
        .collect();

    for letter in old_layout {
        let rule: [char; 2] = match old_magic_rules.get(&letter) {
            Some(other_letter) => [letter, *other_letter],
            None => [letter, letter],
        };
        new_corpus = new_corpus.replace(&format!("{letter}*"), &rule.iter().collect::<String>());
    }
    new_corpus = new_corpus.replace("_", " ");

    println!("{new_corpus}");
}

#[cfg(test)]
mod tests {
    use ahash::AHashMap;

    use crate::{load_layout, load_layout_letters, load_magic_rules};

    #[test]
    fn test_load_letters() {
        let layout_string = load_layout_letters("whirl.txt");
        let expected_layout_string = "qgdfvzluoy\nnsthm'reai\nbcpwkxj;.,\n_*\nwh\ny,\nue\ngs\n'r\n";
        assert_eq!(layout_string, expected_layout_string);
    }
    #[test]
    fn test_load_layout() {
        let layout_array = load_layout("whirl.txt");
        let expected_layout_array = [
            'q', 'g', 'd', 'f', 'v', 'z', 'l', 'u', 'o', 'y', 'n', 's', 't', 'h', 'm', '\'', 'r',
            'e', 'a', 'i', 'b', 'c', 'p', 'w', 'k', 'x', 'j', ';', '.', ',', '_', '*',
        ];
        assert_eq!(layout_array, expected_layout_array);
    }
    #[test]
    fn test_load_magic_rules() {
        let rules = load_magic_rules("whirl.txt");
        let expected_rules =
            AHashMap::from([('w', 'h'), ('u', 'e'), ('g', 's'), ('y', ','), ('\'', 'r')]);
        assert_eq!(rules, expected_rules);
    }
}
