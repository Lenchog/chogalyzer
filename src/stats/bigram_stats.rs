use ahash::AHashMap;

use crate::Finger;
use crate::Key;
use crate::Stats;

pub fn bigram_stats(
    key1: &Key,
    key2: &Key,
    command: &String,
    mut stats: Stats,
    finger_weights: &AHashMap<Finger, i64>,
) -> (Stats, bool, bool, u32) {
    let mut insert_bigram = false;
    let mut bad_bigram = false;
    let mut bigram_weight = 0;

    stats.bigrams += 1;
    if sf(key1, key2) {
        stats.sfb += 1;
        let distance: i64 = if !(key1.lateral || key2.lateral) {
            key1.row as i64 - key2.row as i64
        } else if key1.row == key2.row {
            1
        } else {
            (((key1.row as i64 - key2.row as i64) * (key1.row as i64 - key2.row as i64) + 1) as f64)
                .sqrt() as i64
        };
        stats.fspeed += 5 * finger_weights[&key1.finger] * distance.abs();
        bigram_weight += 5 * finger_weights[&key1.finger] * distance.abs();
        bad_bigram = true;
        if command == "sfb" {
            insert_bigram = true;
        }
    } else if key1 == key2 {
        stats.sfr += 1;
        stats.fspeed += 2 * finger_weights[&key1.finger];
        bigram_weight += 2 * finger_weights[&key1.finger];
        bad_bigram = true;
        if command == "sfr" {
            insert_bigram = true;
        }
    } else {
        if ls(key1, key2) {
            stats.lsb += 1;
            bad_bigram = true;
            bigram_weight += 30;
            if command == "lsb" {
                insert_bigram = true;
            }
        }
        if scissor(key1, key2) == 1 {
            stats.hsb += 1;
            bad_bigram = true;
            bigram_weight += 30;
            if command == "hsb" {
                insert_bigram = true;
            }
        }
        else if scissor(key1, key2) == 2 {
            stats.fsb += 1;
            bad_bigram = true;
            bigram_weight += 90;
            if command == "fsb" {
                insert_bigram = true;
            }
        }
    }
    (
        stats,
        insert_bigram,
        bad_bigram,
        bigram_weight.try_into().unwrap(),
    )
}

pub fn skipgram_stats(
    key1: &Key,
    key2: &Key,
    epic_key1: &Key,
    command: &String,
    mut stats: Stats,
    finger_weights: &AHashMap<Finger, i64>,
) -> (Stats, bool) {
    let mut insert_ngram = false;
    stats.skipgrams += 1;
    if sf(key1, key2) {
        let distance: i64 = key1.row as i64 - key2.row as i64;
        stats.fspeed += distance.abs() * finger_weights[&key1.finger];
        stats.sfs += 1;
        if command == "sfs" {
            insert_ngram = true;
        }
    } else {
        if ls(key1, key2) {
            stats.lss += 1;
            if command == "lss" {
                insert_ngram = true;
            }
        }
        if scissor(key1, key2) == 1 {
            stats.hss += 1;
            if command == "hss" {
                insert_ngram = true;
            }
        }
        else if scissor(key1, key2) == 2 {
            stats.fss += 1;
            if command == "fss" {
                insert_ngram = true;
            }
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
        if scissor(key2, epic_key1) == 2 && epic_key1 != key2 {
            stats.fss += 1;
            if command == "fss" {
                insert_ngram = true;
            }
        }
    }

    (stats, insert_ngram)
}
fn sf(key1: &Key, key2: &Key) -> bool {
    if key1.finger == key2.finger && key1.hand == key2.hand && key1 != key2 {
        return true;
    }
    false
}

fn ls(key1: &Key, key2: &Key) -> bool {
    if (key1.lateral || key2.lateral)
        && key1.hand == key2.hand
        && key1.finger != Finger::Thumb
        && key2.finger != Finger::Thumb
    {
        return true;
    }
    false
}

fn scissor(key1: &Key, key2: &Key) -> u8 {
    let distance: u8 = (i64::from(key1.row) - i64::from(key2.row)).abs().try_into().expect("invalid distance");
    if key1.hand == key2.hand
        && key1.finger != key2.finger
        && (((key1.finger == Finger::Pinky || key1.finger == Finger::Index)
            && (key2.finger == Finger::Middle || key2.finger == Finger::Ring))
        || ((key2.finger == Finger::Pinky || key2.finger == Finger::Index)
            && (key1.finger == Finger::Middle || key1.finger == Finger::Ring)))
    {
        return distance;
    }
    0
}
