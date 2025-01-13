use std::{collections::HashMap, fs};
use tabled::{settings::Style, builder::Builder};
use clap::Parser;

#[derive(PartialEq)]
struct Key {
    hand: u8,
    finger: Finger,
    row: u8,
    lateral: bool,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Trigram {
    Inroll,
    Outroll,
    Alt,
    InThreeRoll,
    OutThreeRoll,
    WeakRed,
    Red,
    ThumbStat,
    SF,
}

#[derive(PartialEq, PartialOrd)]
enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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

    let layout = HashMap::from([
        // LH top row
        (
            layout_raw[0],
            Key {
                hand: 0,
                finger: Finger::Pinky,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[1],
            Key {
                hand: 0,
                finger: Finger::Ring,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[2],
            Key {
                hand: 0,
                finger: Finger::Middle,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[3],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[4],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 0,
                lateral: true,
            },
        ),
        // RH top row
        (
            layout_raw[5],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 0,
                lateral: true,
            },
        ),
        (
            layout_raw[6],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[7],
            Key {
                hand: 1,
                finger: Finger::Middle,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[8],
            Key {
                hand: 1,
                finger: Finger::Ring,
                row: 0,
                lateral: false,
            },
        ),
        (
            layout_raw[9],
            Key {
                hand: 1,
                finger: Finger::Pinky,
                row: 0,
                lateral: false,
            },
        ),
        // LH middle row
        (
            layout_raw[10],
            Key {
                hand: 0,
                finger: Finger::Pinky,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[11],
            Key {
                hand: 0,
                finger: Finger::Ring,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[12],
            Key {
                hand: 0,
                finger: Finger::Middle,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[13],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[14],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 1,
                lateral: true,
            },
        ),
        // RH middle row
        (
            layout_raw[15],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 1,
                lateral: true,
            },
        ),
        (
            layout_raw[16],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[17],
            Key {
                hand: 1,
                finger: Finger::Middle,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[18],
            Key {
                hand: 1,
                finger: Finger::Ring,
                row: 1,
                lateral: false,
            },
        ),
        (
            layout_raw[19],
            Key {
                hand: 1,
                finger: Finger::Pinky,
                row: 1,
                lateral: false,
            },
        ),
        // LH bottom row
        (
            layout_raw[20],
            Key {
                hand: 0,
                finger: Finger::Pinky,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[21],
            Key {
                hand: 0,
                finger: Finger::Ring,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[22],
            Key {
                hand: 0,
                finger: Finger::Middle,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[23],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[24],
            Key {
                hand: 0,
                finger: Finger::Index,
                row: 2,
                lateral: true,
            },
        ),
        // RH bottom row
        (
            layout_raw[25],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 2,
                lateral: true,
            },
        ),
        (
            layout_raw[26],
            Key {
                hand: 1,
                finger: Finger::Index,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[27],
            Key {
                hand: 1,
                finger: Finger::Middle,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[28],
            Key {
                hand: 1,
                finger: Finger::Ring,
                row: 2,
                lateral: false,
            },
        ),
        (
            layout_raw[29],
            Key {
                hand: 1,
                finger: Finger::Pinky,
                row: 2,
                lateral: false,
            },
        ),
        // Thumb keys
        (
            layout_raw[30],
            Key {
                hand: 0,
                finger: Finger::Thumb,
                row: 3,
                lateral: false,
            },
        ),
        (
            layout_raw[31],
            Key {
                hand: 1,
                finger: Finger::Thumb,
                row: 3,
                lateral: false,
            },
        ),
    ]);

    let [mut previous_letter, mut skip_previous_letter, mut epic_previous_letter] = ['⎵'; 3];
    let [mut sfb, mut sfs, mut lsb, mut lss, mut fsb, mut fss, mut alt, mut inroll, mut outroll, mut inthreeroll, mut outthreeroll, mut red, mut weak_red, mut thumb_stat, mut trigram_sf, mut bigrams, mut skipgrams, mut trigrams] = [0; 18];
    let mut ngram_table: HashMap<[char; 3], u32> = HashMap::new();

    for letter in corpus.chars() {
        let key = &layout[&letter];
        let previous_key = &layout[&previous_letter];
        let skip_previous_key = &layout[&skip_previous_letter];
        let epic_previous_key = &layout[&epic_previous_letter];

        if previous_letter != '⎵' && letter != '⎵' {
            bigrams += 1;
            if sf(key, previous_key) {
                sfb += 1;
                if args.command == "sfb" {
                    *ngram_table.entry([previous_letter, letter, ' ']).or_insert(0) += 1;
                }
            }
            if ls(key, previous_key) {
                lsb += 1;
                if args.command == "lsb" {
                    *ngram_table.entry([previous_letter, letter, ' ']).or_insert(0) += 1;
                }
            }
            if fs(key, previous_key) {
                fsb += 1;
                if args.command == "fsb" {
                    *ngram_table.entry([previous_letter, letter, ' ']).or_insert(0) += 1;
                }
            }
        }
        if skip_previous_letter != '⎵' && letter != '⎵' {
            skipgrams += 1;
            if sf(key, skip_previous_key) {
                sfs += 1;
                if args.command == "sfs" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
            if ls(key, skip_previous_key) {
                lss += 1;
                if args.command == "lss" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
            if fs(key, skip_previous_key) {
                fss += 1;
                if args.command == "fss" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
        }

        if INCLUDE_SPACE || (skip_previous_letter != '⎵' && previous_letter != '⎵' && letter != '⎵')
        {
            trigrams += 1;
            //TODO *ngram_table .entry([skip_previous_letter, previous_letter, letter]) .or_insert(0) += 1;
        }
        if key.hand == epic_previous_key.hand {
            skipgrams += 1;
            if sf(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                sfs += 1;
                if args.command == "sfs" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
            if ls(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                lss += 1;
                if args.command == "lss" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
            if fs(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                fss += 1;
                if args.command == "fss" {
                    *ngram_table.entry([previous_letter, '_', letter]).or_insert(0) += 1;
                }
            }
        } 
        match trigram_stat(skip_previous_key, previous_key, key) {
            Trigram::Inroll => {
                inroll += 1;
                if args.command == "inroll" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::Outroll => {
                outroll += 1;
                if args.command == "outroll" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::Alt => {
                alt += 1;
                if args.command == "alt" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::InThreeRoll => {
                inthreeroll += 1;
                if args.command == "inthreeroll" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::OutThreeRoll => {
                outthreeroll += 1;
                if args.command == "outthreeroll" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::Red => {
                red += 1;
                if args.command == "red" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::WeakRed => {
                weak_red += 1;
                if args.command == "weak_red" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::ThumbStat => {
                thumb_stat += 1;
                if args.command == "thumb_stat" {
                    *ngram_table.entry([skip_previous_letter, previous_letter, letter]).or_insert(0) += 1;
                }
            }
            Trigram::SF => {
                trigram_sf += 1;
            }
        }
        epic_previous_letter = letter;
        skip_previous_letter = previous_letter;
        previous_letter = letter;
    }

    if !(INCLUDE_THUMB_ALT || INCLUDE_THUMB_ROLL) {
        trigrams -= thumb_stat;
    }
    let sfbpercent = sfb as f32 * 100.0 / bigrams as f32;
    let sfspercent = sfs as f32 * 100.0 / skipgrams as f32;
    let lsbpercent = lsb as f32 * 100.0 / bigrams as f32;
    let lsspercent = lss as f32 * 100.0 / skipgrams as f32;
    let fsbpercent = fsb as f32 * 100.0 / bigrams as f32;
    let fsspercent = fss as f32 * 100.0 / skipgrams as f32;
    let altpercent = alt as f32 * 100.0 / trigrams as f32;
    let inrollpercent = inroll as f32 * 100.0 / trigrams as f32;
    let outrollpercent = outroll as f32 * 100.0 / trigrams as f32;
    let inthreerollpercent = inthreeroll as f32 * 100.0 / trigrams as f32;
    let outthreerollpercent = outthreeroll as f32 * 100.0 / trigrams as f32;
    let weakredpercent = weak_red as f32 * 100.0 / trigrams as f32;
    let redpercent =
        red as f32 * 100.0 / trigrams as f32;
    let trigramsfpercent =
        trigrams as f32 * 100.0 / trigrams as f32;
    let thumbstatpercent = thumb_stat as f32 * 100.0 / trigrams as f32;

    let mut ngram_vec: Vec<([char; 3], u32)> = ngram_table.into_iter().collect();

    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => println!("SFB%: {}\nSFS%: {}\nLSB%: {}\nLSS%: {}\nFSB%: {}\nFSS%: {}\nAlt%: {}\nInroll%: {}\nOutroll%: {}\nIn3Roll%: {}\nOut3Roll%: {}\nWeak Red%: {}\nRed%: {}\nSF%: {}\nThumb Stat%: {}\n", sfbpercent, sfspercent, lsbpercent, lsspercent, fsbpercent, fsspercent, altpercent, inrollpercent, outrollpercent, inthreerollpercent, outthreerollpercent, weakredpercent, redpercent, trigramsfpercent, thumbstatpercent),
        "sfb" => print_ngrams(ngram_vec, bigrams, "SFB".to_string()),
        "sfs" => print_ngrams(ngram_vec, skipgrams, "SFS".to_string()),
        "lsbs" => print_ngrams(ngram_vec, bigrams, "LSB".to_string()),
        "lss" => print_ngrams(ngram_vec, skipgrams, "LSS".to_string()),
        "fsb" => print_ngrams(ngram_vec, bigrams, "FSB".to_string()),
        "fss" => print_ngrams(ngram_vec, skipgrams, "FSS".to_string()),
        "alt" => print_ngrams(ngram_vec, trigrams, "Alt".to_string()),
        "inroll" => print_ngrams(ngram_vec, trigrams, "Inroll".to_string()),
        "outroll" => print_ngrams(ngram_vec, trigrams, "Outroll".to_string()),
        "inthreeroll" => print_ngrams(ngram_vec, trigrams, "Inthreeroll".to_string()),
        "outthreeroll" => print_ngrams(ngram_vec, trigrams, "Outthreeroll".to_string()),
        "red" => print_ngrams(ngram_vec, trigrams, "Red".to_string()),
        "weak" => print_ngrams(ngram_vec, trigrams, "Weak".to_string()),
        "thumb" => print_ngrams(ngram_vec, trigrams, "Thumb".to_string()),
        "bigrams" => print_ngrams(ngram_vec, bigrams, "Bigrams".to_string()),
        "skipgrams" => print_ngrams(ngram_vec, skipgrams, "Skipgrams".to_string()),
        "trigrams" => print_ngrams(ngram_vec, trigrams, "Trigrams".to_string()),
        _ => println!("invalid command")
    }
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
    if key1.finger == key2.finger && key1.hand == key2.hand && key1.row == key2.row {
        return true;
    }
    false
}

fn trigram_stat(key1: &Key, key2: &Key, key3: &Key) -> Trigram {
    if sfr(key1, key2) || sf(key1, key2) || sfr(key2, key3) || sf(key2, key3) {
        return Trigram::SF;
    }
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
        return Trigram::ThumbStat;
    }
}

fn roll(key1: &Key, key2: &Key) -> Trigram {
    if !INCLUDE_THUMB_ROLL && (key1.finger == Finger::Thumb || key2.finger == Finger::Thumb) {
        return Trigram::ThumbStat;
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
        return Trigram::ThumbStat;
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

fn print_ngrams(vec: Vec<([char; 3], u32)>, ngrams: u32, title: String) {
    let min_range = 0;
    let max_range = 10;
    let mut builder = Builder::default();
    builder.push_record([title, "Frequency".to_string()]);
    for line in vec.iter().take(max_range).skip(min_range) { 
        builder.push_record([line.0.iter().collect(), (line.1 as f32 / ngrams as f32 * 100.0).to_string()]);
    }
    let mut table = builder.build();
    table.with(Style::sharp());
    println!("{}", table);
}

