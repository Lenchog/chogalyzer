use clap::Parser;
use std::{collections::HashMap, fs};
use rand::prelude::*;

mod output;

mod stats;

#[derive(PartialEq)]
pub struct Key {
    hand: u8,
    finger: Finger,
    row: u8,
    lateral: bool,
}

#[derive(PartialEq, PartialOrd)]
enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Default, Debug)]
pub struct Stats {
    sfb: u32,
    sfs: u32,
    lsb: u32,
    lss: u32,
    fsb: u32,
    fss: u32,
    inroll: u32,
    outroll: u32,
    alt: u32,
    inthreeroll: u32,
    outthreeroll: u32,
    weak_red: u32,
    red: u32,
    thumb_stat: u32,
    pub bigrams: u32,
    pub skipgrams: u32,
    pub trigrams: u32,
    pub ngram_table: HashMap<[char; 3], u32>,
}

#[derive(Parser, Debug)]

struct Args {
    #[arg(short, long, default_value = "whirl.txt")]
    layout: String,

    #[arg(short, long, default_value = "mr.txt")]
    corpus: String,

    #[arg(default_value = "analyze")]
    command: String,
}

const INCLUDE_THUMB_ALT: bool = true;
const INCLUDE_THUMB_ROLL: bool = true;
const INCLUDE_SPACE: bool = true;

fn main() {
    let args = Args::parse();

    let layout_letters: String = fs::read_to_string(args.layout)
        .expect("couldn't read layout")
        .replace(" ", "")
        .replace("_", "⎵")
        .chars()
        .collect();


    // has to be 37 because ⎵ is a few extra bytes
    let layout_raw: [char; 32] = layout_letters[..37].replace("\n", "").chars().collect::<Vec<char>>().try_into().expect("couldn't read layout");

    let magic_rules = layout_letters[38..].split("\n");

    let mut corpus: String = fs::read_to_string(args.corpus)
        .expect("error reading corpus")
        .to_lowercase()
        .replace("\n", "⎵")
        .replace(" ", "⎵")
        .chars()
        .filter(|ch| layout_raw.contains(ch))
        .collect();

    for rule in magic_rules {
        if !rule.is_empty() {
            corpus = corpus.replace(rule, &(rule.chars().next().unwrap().to_string() + "*"))
        }
    }

    let stats = stats::analyze(corpus.clone(), &layout_raw_to_table(&layout_raw), &args.command);

    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => output::print_stats(stats, layout_raw),
        "generate" => {
            let layout = generate(0, 1000, layout_raw, &corpus);
            output::print_stats(stats::analyze(corpus, &layout_raw_to_table(&layout), &"generate".to_string()), layout)
        },
        "sfb" => output::print_ngrams(ngram_vec, stats.bigrams, "SFB".to_string()),
        "sfs" => output::print_ngrams(ngram_vec, stats.skipgrams, "SFS".to_string()),
        "lsbs" => output::print_ngrams(ngram_vec, stats.bigrams, "LSB".to_string()),
        "lss" => output::print_ngrams(ngram_vec, stats.skipgrams, "LSS".to_string()),
        "fsb" => output::print_ngrams(ngram_vec, stats.bigrams, "FSB".to_string()),
        "fss" => output::print_ngrams(ngram_vec, stats.skipgrams, "FSS".to_string()),
        "alt" => output::print_ngrams(ngram_vec, stats.trigrams, "Alt".to_string()),
        "inroll" => output::print_ngrams(ngram_vec, stats.trigrams, "Inroll".to_string()),
        "outroll" => output::print_ngrams(ngram_vec, stats.trigrams, "Outroll".to_string()),
        "inthreeroll" => {
            output::print_ngrams(ngram_vec, stats.trigrams, "Inthreeroll".to_string())
        }
        "outthreeroll" => {
            output::print_ngrams(ngram_vec, stats.trigrams, "Outthreeroll".to_string())
        }
        "red" => output::print_ngrams(ngram_vec, stats.trigrams, "Red".to_string()),
        "weak" => output::print_ngrams(ngram_vec, stats.trigrams, "Weak".to_string()),
        "thumb" => output::print_ngrams(ngram_vec, stats.trigrams, "Thumb".to_string()),
        "bigrams" => output::print_ngrams(ngram_vec, stats.bigrams, "Bigrams".to_string()),
        "skipgrams" => {
            output::print_ngrams(ngram_vec, stats.skipgrams, "Skipgrams".to_string())
        }
        "trigrams" => {
            output::print_ngrams(ngram_vec, stats.trigrams, "Trigrams".to_string())
        }
        _ => println!("invalid command"),
    }
}

fn generate(iterations: u32, max_iterations: u32, old_layout: [char; 32], corpus: &String) -> [char; 32] {
    println!("{}% done", iterations as f32 / max_iterations as f32 * 100.0);
    if iterations == max_iterations {
        old_layout
    }
    else {
        let mut rng = rand::thread_rng();
        let mut new_layout: [char; 32] = old_layout;
        new_layout.swap(rng.gen_range(0..old_layout.len()), rng.gen_range(0..old_layout.len()));
        if stats::analyze(corpus.to_string(), &layout_raw_to_table(&old_layout), &"generate".to_string()).sfb > stats::analyze(corpus.to_string(), &layout_raw_to_table(&new_layout), &"generate".to_string()).sfb {
            generate(iterations + 1, max_iterations, new_layout, corpus)
        }
        else { 
            generate(iterations + 1, max_iterations, old_layout, corpus) 
        }
    }
}

fn layout_raw_to_table(layout_raw: &[char; 32]) -> HashMap<char, Key> {
    #[rustfmt::skip]
    return HashMap::from([
        // LH top row
        ( layout_raw[0], Key { hand: 0, finger: Finger::Pinky, row: 0, lateral: false, },),
        ( layout_raw[1], Key { hand: 0, finger: Finger::Ring, row: 0, lateral: false, },),
        ( layout_raw[2], Key { hand: 0, finger: Finger::Middle, row: 0, lateral: false, },),
        ( layout_raw[3], Key { hand: 0, finger: Finger::Index, row: 0, lateral: false, },),
        ( layout_raw[4], Key { hand: 0, finger: Finger::Index, row: 0, lateral: true, },),
        // RH top row
        ( layout_raw[5], Key { hand: 1, finger: Finger::Index, row: 0, lateral: true, },),
        ( layout_raw[6], Key { hand: 1, finger: Finger::Index, row: 0, lateral: false, },),
        ( layout_raw[7], Key { hand: 1, finger: Finger::Middle, row: 0, lateral: false, },),
        ( layout_raw[8], Key { hand: 1, finger: Finger::Ring, row: 0, lateral: false, },),
        ( layout_raw[9], Key { hand: 1, finger: Finger::Pinky, row: 0, lateral: false, },),
        // LH middle row
        ( layout_raw[10], Key { hand: 0, finger: Finger::Pinky, row: 1, lateral: false, },),
        ( layout_raw[11], Key { hand: 0, finger: Finger::Ring, row: 1, lateral: false, },),
        ( layout_raw[12], Key { hand: 0, finger: Finger::Middle, row: 1, lateral: false, },),
        ( layout_raw[13], Key { hand: 0, finger: Finger::Index, row: 1, lateral: false, },),
        ( layout_raw[14], Key { hand: 0, finger: Finger::Index, row: 1, lateral: true, },),
        // RH middle row
        ( layout_raw[15], Key { hand: 1, finger: Finger::Index, row: 1, lateral: true, },),
        ( layout_raw[16], Key { hand: 1, finger: Finger::Index, row: 1, lateral: false, },),
        ( layout_raw[17], Key { hand: 1, finger: Finger::Middle, row: 1, lateral: false, },),
        ( layout_raw[18], Key { hand: 1, finger: Finger::Ring, row: 1, lateral: false, },),
        ( layout_raw[19], Key { hand: 1, finger: Finger::Pinky, row: 1, lateral: false, },),
        // LH bottom row
        ( layout_raw[20], Key { hand: 0, finger: Finger::Pinky, row: 2, lateral: false, },),
        ( layout_raw[21], Key { hand: 0, finger: Finger::Ring, row: 2, lateral: false, },),
        ( layout_raw[22], Key { hand: 0, finger: Finger::Middle, row: 2, lateral: false, },),
        ( layout_raw[23], Key { hand: 0, finger: Finger::Index, row: 2, lateral: false, },),
        ( layout_raw[24], Key { hand: 0, finger: Finger::Index, row: 2, lateral: true, },),
        // RH bottom row
        ( layout_raw[25], Key { hand: 1, finger: Finger::Index, row: 2, lateral: true, },),
        ( layout_raw[26], Key { hand: 1, finger: Finger::Index, row: 2, lateral: false, },),
        ( layout_raw[27], Key { hand: 1, finger: Finger::Middle, row: 2, lateral: false, },),
        ( layout_raw[28], Key { hand: 1, finger: Finger::Ring, row: 2, lateral: false, },),
        ( layout_raw[29], Key { hand: 1, finger: Finger::Pinky, row: 2, lateral: false, },),
        // Thumb keys
        ( layout_raw[30], Key { hand: 0, finger: Finger::Thumb, row: 3, lateral: false, },),
        ( layout_raw[31], Key { hand: 1, finger: Finger::Thumb, row: 3, lateral: false, },),
    ]);
}
