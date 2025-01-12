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
    let [mut sfb, mut sfs, mut lsb, mut lss, mut fsb, mut fss, mut bigrams, mut skipgrams, mut trigrams] =
        [0; 9];
    let mut sfb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut sfs_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut lsb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut lss_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut fsb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut fss_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut bigrams_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut skipgrams_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut trigrams_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut trigrams_mega_table: HashMap<Trigram, (u32, HashMap<[char; 3], u32>)> = HashMap::new();

    for letter in corpus.chars() {
        let key = &layout[&letter];
        let previous_key = &layout[&previous_letter];
        let skip_previous_key = &layout[&skip_previous_letter];
        let epic_previous_key = &layout[&epic_previous_letter];

        if previous_letter != '⎵' && letter != '⎵' {
            bigrams += 1;
            *bigrams_table.entry([previous_letter, letter]).or_insert(0) += 1;
            if sf(key, previous_key) {
                sfb += 1;
                *sfb_table.entry([previous_letter, letter]).or_insert(0) += 1;
            }
            if ls(key, previous_key) {
                lsb += 1;
                *lsb_table.entry([previous_letter, letter]).or_insert(0) += 1;
            }
            if fs(key, previous_key) {
                fsb += 1;
                *fsb_table.entry([previous_letter, letter]).or_insert(0) += 1;
            }
        }
        if skip_previous_letter != '⎵' && letter != '⎵' {
            skipgrams += 1;
            *skipgrams_table
                .entry([skip_previous_letter, letter])
                .or_insert(0) += 1;
            if sf(key, skip_previous_key) {
                sfs += 1;
                *sfs_table.entry([skip_previous_letter, letter]).or_insert(0) += 1;
            }
            if ls(key, skip_previous_key) {
                lss += 1;
                *lss_table.entry([skip_previous_letter, letter]).or_insert(0) += 1;
            }
            if fs(key, skip_previous_key) {
                fss += 1;
                *fss_table.entry([skip_previous_letter, letter]).or_insert(0) += 1;
            }
        }

        if INCLUDE_SPACE || (skip_previous_letter != '⎵' && previous_letter != '⎵' && letter != '⎵')
        {
            trigrams += 1;
            *trigrams_table
                .entry([skip_previous_letter, previous_letter, letter])
                .or_insert(0) += 1;
            trigrams_mega_table
                .entry(trigram_stat(skip_previous_key, previous_key, key))
                .or_default()
                .0 += 1;
            trigrams_mega_table
                .entry(trigram_stat(skip_previous_key, previous_key, key))
                .or_default()
                .1 = trigrams_table.clone()
        }
        if key.hand == epic_previous_key.hand {
            skipgrams += 1;
            if sf(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                sfs += 1
            }
            if ls(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                lss += 1
            }
            if fs(key, epic_previous_key)
                && epic_previous_letter != skip_previous_letter
                && epic_previous_letter != previous_letter
            {
                fss += 1
            }
        }
        epic_previous_letter = letter;
        skip_previous_letter = previous_letter;
        previous_letter = letter;
    }

    if !(INCLUDE_THUMB_ALT || INCLUDE_THUMB_ROLL) {
        trigrams -= trigrams_mega_table.entry(Trigram::ThumbStat).or_default().0;
    }
    let sfbpercent = sfb as f32 * 100.0 / bigrams as f32;
    let sfspercent = sfs as f32 * 100.0 / skipgrams as f32;
    let lsbpercent = lsb as f32 * 100.0 / bigrams as f32;
    let lsspercent = lss as f32 * 100.0 / skipgrams as f32;
    let fsbpercent = fsb as f32 * 100.0 / bigrams as f32;
    let fsspercent = fss as f32 * 100.0 / skipgrams as f32;
    let altpercent =
        trigrams_mega_table.entry(Trigram::Alt).or_default().0 as f32 * 100.0 / trigrams as f32;
    let inrollpercent =
        trigrams_mega_table.entry(Trigram::Inroll).or_default().0 as f32 * 100.0 / trigrams as f32;
    let outrollpercent =
        trigrams_mega_table.entry(Trigram::Outroll).or_default().0 as f32 * 100.0 / trigrams as f32;
    let inthreerollpercent = trigrams_mega_table
        .entry(Trigram::InThreeRoll)
        .or_default()
        .0 as f32
        * 100.0
        / trigrams as f32;
    let outthreerollpercent = trigrams_mega_table
        .entry(Trigram::OutThreeRoll)
        .or_default()
        .0 as f32
        * 100.0
        / trigrams as f32;
    let weakredpercent =
        trigrams_mega_table.entry(Trigram::WeakRed).or_default().0 as f32 * 100.0 / trigrams as f32;
    let redpercent =
        trigrams_mega_table.entry(Trigram::Red).or_default().0 as f32 * 100.0 / trigrams as f32;
    let trigramsfpercent =
        trigrams_mega_table.entry(Trigram::SF).or_default().0 as f32 * 100.0 / trigrams as f32;
    let thumbstatpercent = trigrams_mega_table.entry(Trigram::ThumbStat).or_default().0 as f32
        * 100.0
        / trigrams as f32;

    let mut sfb_vec: Vec<([char; 2], u32)> = sfb_table.into_iter().collect();
    let mut sfs_vec: Vec<([char; 2], u32)> = sfs_table.into_iter().collect();
    let mut lsb_vec: Vec<([char; 2], u32)> = lsb_table.into_iter().collect();
    let mut lss_vec: Vec<([char; 2], u32)> = lss_table.into_iter().collect();
    let mut fsb_vec: Vec<([char; 2], u32)> = fsb_table.into_iter().collect();
    let mut fss_vec: Vec<([char; 2], u32)> = fss_table.into_iter().collect();

    let mut alt_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::Alt)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut inroll_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::Inroll)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut outroll_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::Outroll)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut inthreeroll_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::InThreeRoll)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut outthreeroll_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::OutThreeRoll)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut red_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::Red)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut weak_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::WeakRed)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();
    let mut thumb_vec: Vec<([char; 3], u32)> = trigrams_mega_table
        .remove(&Trigram::ThumbStat)
        .unwrap_or_default()
        .1
        .into_iter()
        .collect();

    let mut trigrams_vec: Vec<([char; 3], u32)> = trigrams_table.into_iter().collect();
    let mut bigrams_vec: Vec<([char; 2], u32)> = bigrams_table.into_iter().collect();
    let mut skipgrams_vec: Vec<([char; 2], u32)> = skipgrams_table.into_iter().collect();

    sfb_vec.sort_by(|a, b| b.1.cmp(&a.1));
    sfs_vec.sort_by(|a, b| b.1.cmp(&a.1));
    lsb_vec.sort_by(|a, b| b.1.cmp(&a.1));
    lss_vec.sort_by(|a, b| b.1.cmp(&a.1));
    fsb_vec.sort_by(|a, b| b.1.cmp(&a.1));
    fss_vec.sort_by(|a, b| b.1.cmp(&a.1));
    alt_vec.sort_by(|a, b| b.1.cmp(&a.1));
    inroll_vec.sort_by(|a, b| b.1.cmp(&a.1));
    outroll_vec.sort_by(|a, b| b.1.cmp(&a.1));
    inthreeroll_vec.sort_by(|a, b| b.1.cmp(&a.1));
    outthreeroll_vec.sort_by(|a, b| b.1.cmp(&a.1));
    red_vec.sort_by(|a, b| b.1.cmp(&a.1));
    weak_vec.sort_by(|a, b| b.1.cmp(&a.1));
    thumb_vec.sort_by(|a, b| b.1.cmp(&a.1));
    thumb_vec.sort_by(|a, b| b.1.cmp(&a.1));
    bigrams_vec.sort_by(|a, b| b.1.cmp(&a.1));
    skipgrams_vec.sort_by(|a, b| b.1.cmp(&a.1));
    trigrams_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => println!("SFB%: {}\nSFS%: {}\nLSB%: {}\nLSS%: {}\nFSB%: {}\nFSS%: {}\nAlt%: {}\nInroll%: {}\nOutroll%: {}\nIn3Roll%: {}\nOut3Roll%: {}\nWeak Red%: {}\nRed%: {}\nSF%: {}\nThumb Stat%: {}\n", sfbpercent, sfspercent, lsbpercent, lsspercent, fsbpercent, fsspercent, altpercent, inrollpercent, outrollpercent, inthreerollpercent, outthreerollpercent, weakredpercent, redpercent, trigramsfpercent, thumbstatpercent),
        "sfb" => print_bigrams(sfb_vec, bigrams, "SFB".to_string()),
        "sfs" => print_bigrams(sfs_vec, skipgrams, "SFS".to_string()),
        "lsbs" => print_bigrams(lsb_vec, bigrams, "LSB".to_string()),
        "lss" => print_bigrams(lss_vec, skipgrams, "LSS".to_string()),
        "fsb" => print_bigrams(fsb_vec, bigrams, "FSB".to_string()),
        "fss" => print_bigrams(fss_vec, skipgrams, "FSS".to_string()),
        "alt" => print_trigrams(alt_vec, trigrams, "Alt".to_string()),
        "inroll" => print_trigrams(inroll_vec, trigrams, "Inroll".to_string()),
        "outroll" => print_trigrams(outroll_vec, trigrams, "Outroll".to_string()),
        "inthreeroll" => print_trigrams(inthreeroll_vec, trigrams, "Inthreeroll".to_string()),
        "outthreeroll" => print_trigrams(outthreeroll_vec, trigrams, "Outthreeroll".to_string()),
        "red" => print_trigrams(red_vec, trigrams, "Red".to_string()),
        "weak" => print_trigrams(weak_vec, trigrams, "Weak".to_string()),
        "thumb" => print_trigrams(thumb_vec, trigrams, "Thumb".to_string()),
        "bigrams" => print_bigrams(bigrams_vec, bigrams, "Bigrams".to_string()),
        "skipgrams" => print_bigrams(skipgrams_vec, skipgrams, "Skipgrams".to_string()),
        "trigrams" => print_trigrams(trigrams_vec, trigrams, "Trigrams".to_string()),
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

fn print_trigrams(vec: Vec<([char; 3], u32)>, ngrams: u32, title: String) {
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

fn print_bigrams(vec: Vec<([char; 2], u32)>, ngrams: u32, title: String) {
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
