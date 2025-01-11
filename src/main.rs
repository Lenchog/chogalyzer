use std::{collections::HashMap, fs};

struct Key {
    hand: u8,
    finger: Finger,
    row: u8,
    lateral: bool,
}

enum Trigram {
    Inroll,
    Outroll,
    Alt,
    InThreeRoll,
    OutThreeRoll,
    WeakRed,
    Red,
    ThumbStat,
}

#[derive(PartialEq, PartialOrd)]
enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

fn main() {

    let layout = "whirl";
    let corpus = "mr";

    let layout_letters: Vec<char> =
        fs::read_to_string(layout.to_owned() + ".txt").expect("couldn't read layout")
            .replace(" ", "")
            .replace("\n", "")
            .chars()
            .collect();

    let layout_raw: [char; 32] = layout_letters.try_into().expect("invalid layout");

    let corpus = fs::read_to_string(corpus.to_owned() + ".txt").expect("error reading corpus")
        .to_lowercase()
        .replace("\n", "_")
        .replace(" ", "_")
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

    let [mut previous_letter, mut skip_previous_letter, mut epic_previous_letter] = ['_'; 3];
    let [mut sfb, mut sfs, mut lsb, mut lss, mut fsb, mut fss, mut alt, mut inroll, mut outroll, mut inthreeroll, mut outthreeroll, mut red, mut weak_red, mut thumb_stat, mut bigrams, mut skipgrams, mut trigrams] =
        [0; 17];
    let mut sfb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut sfs_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut lsb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut lss_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut fsb_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut fss_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut alt_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut inroll_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut outroll_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut inthreeroll_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut outthreeroll_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut red_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut weak_red_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut thumb_stat_table: HashMap<[char; 3], u32> = HashMap::new();
    let mut bigrams_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut skipgrams_table: HashMap<[char; 2], u32> = HashMap::new();
    let mut trigrams_table: HashMap<[char; 3], u32> = HashMap::new();

    for letter in corpus.chars() {
        let key = &layout[&letter];
        let previous_key = &layout[&previous_letter];
        let skip_previous_key = &layout[&skip_previous_letter];
        let epic_previous_key = &layout[&epic_previous_letter];

        if previous_letter != '_' && letter != '_' {
            bigrams += 1;
            *bigrams_table
                .entry([previous_letter, letter])
                .or_insert(0) += 1;
            if sf(key, previous_key) {
                sfb += 1;
                *sfb_table
                    .entry([previous_letter, letter])
                    .or_insert(0) += 1;
            }
            if ls(key, previous_key) {
                lsb += 1;
                *lsb_table
                    .entry([previous_letter, letter])
                    .or_insert(0) += 1;
            }
            if fs(key, previous_key) {
                fsb += 1;
                *fsb_table
                    .entry([previous_letter, letter])
                    .or_insert(0) += 1;
            }
        }
        if skip_previous_letter != '_' && letter != '_' {
            skipgrams += 1;
            *skipgrams_table
                .entry([skip_previous_letter, letter])
                .or_insert(0) += 1;
            if sf(key, skip_previous_key) {
                sfs += 1;
                *sfs_table
                    .entry([skip_previous_letter, letter])
                    .or_insert(0) += 1;
            }
            if ls(key, skip_previous_key) {
                lss += 1;
                *lss_table
                    .entry([skip_previous_letter, letter])
                    .or_insert(0) += 1;
            }
            if fs(key, skip_previous_key) {
                fss += 1;
                *fss_table
                    .entry([skip_previous_letter, letter])
                    .or_insert(0) += 1;
            }
        }

        if !(key.finger == Finger::Thumb || previous_key.finger == Finger::Thumb || skip_previous_key.finger == Finger::Thumb) {
            trigrams += 1;
            *trigrams_table
                .entry([skip_previous_letter, previous_letter, letter])
                .or_insert(0) += 1;
            match trigram_stat(key, previous_key, skip_previous_key) {
                Trigram::Inroll => {
                    inroll += 1;
                    *inroll_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::Outroll => {
                    outroll += 1;
                    *outroll_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::Alt => {
                    alt += 1;
                    *alt_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::InThreeRoll => {
                    inthreeroll += 1;
                    *inthreeroll_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::OutThreeRoll => {
                    outthreeroll += 1;
                    *outthreeroll_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::Red => {
                    red += 1;
                    *red_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::WeakRed => {
                    weak_red += 1;
                    *weak_red_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
                Trigram::ThumbStat => {
                    thumb_stat += 1;
                    *thumb_stat_table
                        .entry([skip_previous_letter, previous_letter, letter])
                        .or_insert(0) += 1;
                }
            }
        }

        if key.hand == epic_previous_key.hand {
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
            epic_previous_letter = letter;
        }
        skip_previous_letter = previous_letter;
        previous_letter = letter
    }

    let mut sfb_vec: Vec<([char; 2], u32)> = sfb_table.into_iter().collect();
    let mut sfs_vec: Vec<([char; 2], u32)> = sfs_table.into_iter().collect();
    let mut lsb_vec: Vec<([char; 2], u32)> = lsb_table.into_iter().collect();
    let mut lss_vec: Vec<([char; 2], u32)> = lss_table.into_iter().collect();
    let mut fsb_vec: Vec<([char; 2], u32)> = fsb_table.into_iter().collect();
    let mut fss_vec: Vec<([char; 2], u32)> = fss_table.into_iter().collect();
    let mut alt_vec: Vec<([char; 3], u32)> = alt_table.into_iter().collect();
    let mut inroll_vec: Vec<([char; 3], u32)> = inroll_table.into_iter().collect();
    let mut outroll_vec: Vec<([char; 3], u32)> = outroll_table.into_iter().collect();
    let mut inthreeroll_vec: Vec<([char; 3], u32)> = inthreeroll_table.into_iter().collect();
    let mut outthreeroll_vec: Vec<([char; 3], u32)> = outthreeroll_table.into_iter().collect();
    let mut red_vec: Vec<([char; 3], u32)> = red_table.into_iter().collect();
    let mut weak_vec: Vec<([char; 3], u32)> = weak_red_table.into_iter().collect();
    let mut thumb_vec: Vec<([char; 3], u32)> = thumb_stat_table.into_iter().collect();
    let mut bigrams_vec: Vec<([char; 2], u32)> = bigrams_table.into_iter().collect();
    let mut skipgrams_vec: Vec<([char; 2], u32)> = skipgrams_table.into_iter().collect();
    let mut trigrams_vec: Vec<([char; 3], u32)> = trigrams_table.into_iter().collect();

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
    trigrams -= thumb_stat;
    let sfbpercent = sfb as f64 * 100.0 / bigrams as f64;
    let sfspercent = sfs as f64 * 100.0 / skipgrams as f64;
    let lsbpercent = lsb as f64 * 100.0 / bigrams as f64;
    let lsspercent = lss as f64 * 100.0 / skipgrams as f64;
    let fsbpercent = fsb as f64 * 100.0 / bigrams as f64;
    let fsspercent = fss as f64 * 100.0 / skipgrams as f64;
    let altpercent = alt as f64 * 100.0 / trigrams as f64;
    let inrollpercent = inroll as f64 * 100.0 / trigrams as f64;
    let outrollpercent = outroll as f64 * 100.0 / trigrams as f64;
    let inthreerollpercent = inthreeroll as f64 * 100.0 / trigrams as f64;
    let outthreerollpercent = outthreeroll as f64 * 100.0 / trigrams as f64;
    let weakredpercent = weak_red as f64 * 100.0 / trigrams as f64;
    //let thumbstatpercent = thumb_stat as f64 * 100.0 / trigrams as f64;
    let redpercent = red as f64 * 100.0 / trigrams as f64;

    println!("SFB%: {}\nSFS%: {}\nLSB%: {}\nLSS%: {}\nFSB%: {}\nFSS%: {}\nAlt%: {}\nInroll%: {}\nOutroll%: {}\nInFinger::MiddleRoll%: {}\nOut3Roll%: {}\nWeak Red%: {}\nRed%: {}\nThumb Stats: {}", sfbpercent, sfspercent, lsbpercent, lsspercent, fsbpercent, fsspercent, altpercent, inrollpercent, outrollpercent, inthreerollpercent, outthreerollpercent, weakredpercent, redpercent, thumb_stat);
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

fn trigram_stat(key1: &Key, key2: &Key, key3: &Key) -> Trigram {
    if key2.hand != key1.hand && key2.hand != key3.hand {
        return Trigram::Alt;
    }
    if key1.hand == key2.hand && key2.hand == key3.hand {
        return onehand(key1, key2, key3);
    };
    if key1.hand == key2.hand {
         roll(key1, key2)
    } else {
         roll(key2, key3)
    }
}

fn roll(key1: &Key, key2: &Key) -> Trigram {
    if key1.finger == Finger::Thumb || key2.finger == Finger::Thumb {
        return Trigram::ThumbStat;
    }
    if key1.finger < key2.finger {
        Trigram::Inroll
    } else {
        Trigram::Outroll
    }
}

fn onehand(key1: &Key, key2: &Key, key3: &Key) -> Trigram {
    if key1.finger == Finger::Thumb || key2.finger == Finger::Thumb || key3.finger == Finger::Thumb {
        return Trigram::ThumbStat;
    }
    if key1.finger < key2.finger && key2.finger < key3.finger {
        return Trigram::InThreeRoll;
    }
    if key1.finger > key2.finger && key2.finger > key3.finger {
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
