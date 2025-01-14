use clap::Parser;
use std::{collections::HashMap, fs};

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
    #[arg(short, long, default_value = "whirl")]
    layout: String,

    #[arg(short, long, default_value = "mr")]
    corpus: String,

    #[arg(default_value = "analyze")]
    command: String,
}

const INCLUDE_THUMB_ALT: bool = true;
const INCLUDE_THUMB_ROLL: bool = true;
const INCLUDE_SPACE: bool = true;

fn main() {
    let args = Args::parse();

    let layout_letters: Vec<char> = fs::read_to_string(args.layout + ".txt")
        .expect("couldn't read layout")
        .replace(" ", "")
        .replace("_", "⎵")
        .replace("\n", "")
        .chars()
        .collect();

    let layout_raw: [char; 32] = layout_letters.try_into().expect("invalid layout");

    let corpus = fs::read_to_string(args.corpus + ".txt")
        .expect("error reading corpus")
        .to_lowercase()
        .replace("\n", "⎵")
        .replace(" ", "⎵")
        .chars()
        .filter(|ch| layout_raw.contains(ch))
        .collect::<String>();

    #[rustfmt::skip]
    let layout = HashMap::from([
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

    let stats = crate::stats::analyze(corpus, layout, &args.command);

    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => crate::output::print_stats(stats),
        "sfb" => crate::output::print_ngrams(ngram_vec, stats.bigrams, "SFB".to_string()),
        "sfs" => crate::output::print_ngrams(ngram_vec, stats.skipgrams, "SFS".to_string()),
        "lsbs" => crate::output::print_ngrams(ngram_vec, stats.bigrams, "LSB".to_string()),
        "lss" => crate::output::print_ngrams(ngram_vec, stats.skipgrams, "LSS".to_string()),
        "fsb" => crate::output::print_ngrams(ngram_vec, stats.bigrams, "FSB".to_string()),
        "fss" => crate::output::print_ngrams(ngram_vec, stats.skipgrams, "FSS".to_string()),
        "alt" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Alt".to_string()),
        "inroll" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Inroll".to_string()),
        "outroll" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Outroll".to_string()),
        "inthreeroll" => {
            crate::output::print_ngrams(ngram_vec, stats.trigrams, "Inthreeroll".to_string())
        }
        "outthreeroll" => {
            crate::output::print_ngrams(ngram_vec, stats.trigrams, "Outthreeroll".to_string())
        }
        "red" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Red".to_string()),
        "weak" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Weak".to_string()),
        "thumb" => crate::output::print_ngrams(ngram_vec, stats.trigrams, "Thumb".to_string()),
        "bigrams" => crate::output::print_ngrams(ngram_vec, stats.bigrams, "Bigrams".to_string()),
        "skipgrams" => {
            crate::output::print_ngrams(ngram_vec, stats.skipgrams, "Skipgrams".to_string())
        }
        "trigrams" => {
            crate::output::print_ngrams(ngram_vec, stats.trigrams, "Trigrams".to_string())
        }
        _ => println!("invalid command"),
    }
}

pub mod stats {
    use crate::{Key, Stats, INCLUDE_SPACE, INCLUDE_THUMB_ALT, INCLUDE_THUMB_ROLL};
    use std::collections::HashMap;

    pub fn analyze(corpus: String, layout: HashMap<char, Key>, command: &String) -> Stats {
        use crate::stats::bigram_stats;
        use crate::Stats;

        let [mut previous_letter, mut skip_previous_letter, mut epic_previous_letter] = ['⎵'; 3];
        let mut stats: Stats = Stats::default();
        let mut ngram_table: HashMap<[char; 3], u32> = HashMap::new();

        for letter in corpus.chars() {
            let key = &layout[&letter];
            let previous_key = &layout[&previous_letter];
            let skip_previous_key = &layout[&skip_previous_letter];
            let epic_previous_key = &layout[&epic_previous_letter];

            if INCLUDE_SPACE || (previous_letter != '⎵' && letter != '⎵') {
                let bigram = bigram_stats::bigram_stats(previous_key, key, command, stats);
                stats = bigram.0;
                if bigram.1 {
                    *ngram_table
                        .entry([previous_letter, letter, ' '])
                        .or_insert(0) += 1;
                }
            }

            if INCLUDE_SPACE || skip_previous_letter != '⎵' && letter != '⎵' {
                let skipgram = bigram_stats::skipgram_stats(
                    skip_previous_key,
                    key,
                    epic_previous_key,
                    command,
                    stats,
                );
                stats = skipgram.0;
                if skipgram.1 {
                    *ngram_table
                        .entry([skip_previous_letter, letter, ' '])
                        .or_insert(0) += 1;
                }
            }

            if INCLUDE_SPACE
                || (skip_previous_letter != '⎵' && previous_letter != '⎵' && letter != '⎵')
            {
                let trigram = trigram_stats::trigram_stats(
                    skip_previous_key,
                    previous_key,
                    key,
                    command,
                    stats,
                );
                stats = trigram.0;
                stats.trigrams += 1;
                if trigram.1 {
                    *ngram_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
            }
            epic_previous_letter = letter;
            skip_previous_letter = previous_letter;
            previous_letter = letter;
        }
        if !(INCLUDE_THUMB_ALT || INCLUDE_THUMB_ROLL) {
            stats.trigrams -= stats.thumb_stat;
        }
        stats.ngram_table = ngram_table;
        stats
    }

    pub mod trigram_stats {
        use crate::Finger;
        use crate::Key;
        use crate::Stats;
        use crate::INCLUDE_THUMB_ALT;
        use crate::INCLUDE_THUMB_ROLL;

        #[derive(Debug, Eq, Hash, PartialEq)]
        enum Trigram {
            Inroll,
            Outroll,
            Alt,
            InThreeRoll,
            OutThreeRoll,
            WeakRed,
            Red,
            Other,
        }

        fn trigram_stat(key1: &Key, key2: &Key, key3: &Key) -> Trigram {
            if key2.hand != key1.hand
                && key2.hand != key3.hand
                && (INCLUDE_THUMB_ALT
                    || !(key1.finger == Finger::Thumb
                        || key2.finger == Finger::Thumb
                        || key3.finger == Finger::Thumb))
            {
                return Trigram::Alt;
            }
            if key1.hand == key2.hand && key2.hand == key3.hand {
                return onehand(key1, key2, key3);
            };
            if key1.hand == key2.hand {
                roll(key1, key2)
            } else if key2.hand == key3.hand {
                roll(key2, key3)
            } else {
                return Trigram::Other;
            }
        }

        fn roll(key1: &Key, key2: &Key) -> Trigram {
            if !INCLUDE_THUMB_ROLL && (key1.finger == Finger::Thumb || key2.finger == Finger::Thumb)
            {
                return Trigram::Other;
            }
            if key1.finger > key2.finger {
                return Trigram::Inroll;
            }
            Trigram::Outroll
        }

        fn onehand(key1: &Key, key2: &Key, key3: &Key) -> Trigram {
            if !INCLUDE_THUMB_ROLL
                && (key1.finger == Finger::Thumb
                    || key2.finger == Finger::Thumb
                    || key3.finger == Finger::Thumb)
            {
                return Trigram::Other;
            }
            if key1.finger > key2.finger && key2.finger > key3.finger {
                return Trigram::InThreeRoll;
            }
            if key1.finger < key2.finger && key2.finger < key3.finger {
                return Trigram::OutThreeRoll;
            }
            if key1.finger != Finger::Index
                && key2.finger != Finger::Index
                && key3.finger != Finger::Index
                && key1.finger != Finger::Thumb
                && key2.finger != Finger::Thumb
                && key3.finger != Finger::Thumb
            {
                return Trigram::WeakRed;
            }
            Trigram::Red
        }

        pub fn trigram_stats(
            key1: &Key,
            key2: &Key,
            key3: &Key,
            command: &String,
            mut stats: Stats,
        ) -> (Stats, bool) {
            let mut insert_ngram = false;
            match trigram_stat(key1, key2, key3) {
                Trigram::Inroll => {
                    stats.inroll += 1;
                    if command == "inroll" {
                        insert_ngram = true;
                    }
                }
                Trigram::Outroll => {
                    stats.outroll += 1;
                    if command == "outroll" {
                        insert_ngram = true;
                    }
                }
                Trigram::Alt => {
                    stats.alt += 1;
                    if command == "alt" {
                        insert_ngram = true;
                    }
                }
                Trigram::InThreeRoll => {
                    stats.inthreeroll += 1;
                    if command == "inthreeroll" {
                        insert_ngram = true;
                    }
                }
                Trigram::OutThreeRoll => {
                    stats.outthreeroll += 1;
                    if command == "outthreeroll" {
                        insert_ngram = true;
                    }
                }
                Trigram::Red => {
                    stats.red += 1;
                    if command == "red" {
                        insert_ngram = true;
                    }
                }
                Trigram::WeakRed => {
                    stats.weak_red += 1;
                    if command == "weak_red" {
                        insert_ngram = true;
                    }
                }
                Trigram::Other => {
                    stats.thumb_stat += 1;
                    if command == "thumb_stat" {
                        insert_ngram = true;
                    }
                }
            }
            (stats, insert_ngram)
        }
    }

    pub mod bigram_stats {
        use crate::Finger;
        use crate::Key;
        use crate::Stats;
        pub fn bigram_stats(
            key1: &Key,
            key2: &Key,
            command: &String,
            mut stats: Stats,
        ) -> (Stats, bool) {
            let mut insert_ngram = false;
            stats.bigrams += 1;
            if sf(key1, key2) {
                stats.sfb += 1;
                if command == "sfb" {
                    insert_ngram = true;
                }
            }
            if ls(key1, key2) {
                stats.lsb += 1;
                if command == "lsb" {
                    insert_ngram = true;
                }
            }
            if fs(key1, key2) {
                stats.fsb += 1;
                if command == "fsb" {
                    insert_ngram = true;
                }
            }
            (stats, insert_ngram)
        }

        pub fn skipgram_stats(
            key1: &Key,
            key2: &Key,
            epic_key1: &Key,
            command: &String,
            mut stats: Stats,
        ) -> (Stats, bool) {
            let mut insert_ngram = false;
            stats.skipgrams += 1;
            if sf(key1, key2) {
                stats.sfs += 1;
                if command == "sfs" {
                    insert_ngram = true;
                }
            }
            if ls(key1, key2) {
                stats.lss += 1;
                if command == "lss" {
                    insert_ngram = true;
                }
            }
            if fs(key1, key2) {
                stats.fss += 1;
                if command == "fss" {
                    insert_ngram = true;
                }
            }

            if epic_key1.hand == key2.hand {
                stats.skipgrams += 1;
                if sf(epic_key1, key2) && epic_key1 != key2 {
                    stats.sfs += 1;
                    if command == "sfs" {
                        insert_ngram = true;
                    }
                }
                if ls(key2, epic_key1) && epic_key1 != key2 {
                    stats.lss += 1;
                    if command == "lss" {
                        insert_ngram = true;
                    }
                }
                if fs(key2, epic_key1) && epic_key1 != key2 {
                    stats.fss += 1;
                    if command == "fss" {
                        insert_ngram = true;
                    }
                }
            }

            (stats, insert_ngram)
        }
        fn sf(key1: &Key, key2: &Key) -> bool {
            if key1.finger == key2.finger && key1.hand == key2.hand && key1.row != key2.row {
                return true;
            }
            false
        }

        fn ls(key1: &Key, key2: &Key) -> bool {
            if (key1.lateral || key2.lateral) && key1.hand == key2.hand {
                return true;
            }
            false
        }

        fn fs(key1: &Key, key2: &Key) -> bool {
            if (((key1.finger == Finger::Ring || key1.finger == Finger::Middle)
                && (key2.finger == Finger::Pinky || key2.finger == Finger::Index)
                && (key1.row == 2 && key2.row == 0 || key1.row == 0 && key2.row == 2))
                || ((key2.finger == Finger::Ring || key2.finger == Finger::Middle)
                    && (key1.finger == Finger::Pinky || key1.finger == Finger::Index)
                    && key2.row == 0
                    && key1.row == 2))
                && key1.hand == key2.hand
            {
                return true;
            }
            false
        }

        fn sfr(key1: &Key, key2: &Key) -> bool {
            if key1 == key2 {
                return true;
            }
            false
        }
    }
}

pub mod output {
    use crate::Stats;
    use tabled::{builder::Builder, settings::Style};

    pub fn print_ngrams(vec: Vec<([char; 3], u32)>, ngrams: u32, title: String) {
        let min_range = 0;
        let max_range = 10;
        let mut builder = Builder::default();
        builder.push_record([title, "Frequency".to_string()]);
        for line in vec.iter().take(max_range).skip(min_range) {
            builder.push_record([
                line.0.iter().collect(),
                (line.1 as f32 / ngrams as f32 * 100.0).to_string(),
            ]);
        }
        let mut table = builder.build();
        table.with(Style::sharp());
        println!("{}", table);
    }

    pub fn print_stats(stats: Stats) {
        let sfbpercent = stats.sfb as f32 * 100.0 / stats.bigrams as f32;
        let sfspercent = stats.sfs as f32 * 100.0 / stats.skipgrams as f32;
        let lsbpercent = stats.lsb as f32 * 100.0 / stats.bigrams as f32;
        let lsspercent = stats.lss as f32 * 100.0 / stats.skipgrams as f32;
        let fsbpercent = stats.fsb as f32 * 100.0 / stats.bigrams as f32;
        let fsspercent = stats.fss as f32 * 100.0 / stats.skipgrams as f32;
        let altpercent = stats.alt as f32 * 100.0 / stats.trigrams as f32;
        let inrollpercent = stats.inroll as f32 * 100.0 / stats.trigrams as f32;
        let outrollpercent = stats.outroll as f32 * 100.0 / stats.trigrams as f32;
        let inthreerollpercent = stats.inthreeroll as f32 * 100.0 / stats.trigrams as f32;
        let outthreerollpercent = stats.outthreeroll as f32 * 100.0 / stats.trigrams as f32;
        let weakredpercent = stats.weak_red as f32 * 100.0 / stats.trigrams as f32;
        let redpercent = stats.red as f32 * 100.0 / stats.trigrams as f32;
        println!(
            "{}",
            format_args!(
                concat!(
                    "SFB: {}%\n",
                    "SFS: {}%\n",
                    "LSB: {}%\n",
                    "LSS: {}%\n",
                    "FSB: {}%\n",
                    "FSS: {}%\n",
                    "Alt: {}%\n",
                    "Roll: {}%\n",
                    "  In: {}%\n",
                    "  Out: {}%\n",
                    "Threeroll: {}%\n",
                    "  In: {}%\n",
                    "  Out: {}%\n",
                    "Redirects: {}%\n",
                    "Weak Redirects: {}%\n"
                ),
                sfbpercent,
                sfspercent,
                lsbpercent,
                lsspercent,
                fsbpercent,
                fsspercent,
                altpercent,
                inrollpercent + outrollpercent,
                inrollpercent,
                outrollpercent,
                inthreerollpercent + outthreerollpercent,
                inthreerollpercent,
                outthreerollpercent,
                redpercent + weakredpercent,
                weakredpercent
            )
        )
    }
}
