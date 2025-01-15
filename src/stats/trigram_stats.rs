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
