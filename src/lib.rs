pub mod generation;
pub mod output;
pub mod stats;

use ahash::AHashMap;
use clap::Parser;
use std::fs;

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
    column_pen: i64,
    thumb_stat: u32,
    pub skipgrams: u32,
    pub chars: u32,
    pub ngram_table: AHashMap<[char; 3], u32>,
    pub bad_bigrams: AHashMap<[char; 2], u32>,
}

const INCLUDE_THUMB_ALT: bool = true;
const INCLUDE_THUMB_ROLL: bool = true;

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

fn load_layout_letters(layout: &str) -> String {
    let layout_letters: String = fs::read_to_string("layouts/".to_owned() + layout)
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .chars()
        .collect();
    layout_letters
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
