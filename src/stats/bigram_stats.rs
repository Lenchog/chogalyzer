use ahash::AHashMap;

use crate::Finger;
use crate::Key;
use crate::Stats;

pub fn bigram_stats(
    key1: &Key,
    key2: &Key,
    command: &str,
    stats: &mut Stats,
    finger_weights: &AHashMap<Finger, i64>,
    get_bad_bigrams: bool,
) -> (bool, u32) {
    let mut insert_bigram = false;
    let mut bigram_weight = 0;

    if key1.hand == key2.hand && key1.finger != Finger::Thumb && key2.finger != Finger::Thumb {
        if key1.finger == key2.finger {
            let sfr = key1 == key2;
            if sfr {
                stats.sfr += 1;
            } else {
                stats.sfb += 1;
            }
            let weight = if sfr { 2 } else { 5 };

            let dy = key1.row.abs_diff(key2.row);
            let distance: u8 = if !sfr {
                if key1.lateral == key2.lateral {
                    dy.max(1)
                } else {
                    (dy.pow(2) + 1).isqrt()
                }
            } else {
                1
            };
            let penalty = weight * finger_weights[&key1.finger] * distance as i64;
            stats.fspeed += penalty;
            if get_bad_bigrams {
                bigram_weight += 5 * penalty;
            }
            if (!sfr && command == "sfb") || (sfr && command == "sfr") {
                insert_bigram = true;
            }
        } else {
            if key1.lateral || key2.lateral {
                stats.lsb += 1;
                if get_bad_bigrams {
                    bigram_weight += 15;
                }
                if command == "lsb" {
                    insert_bigram = true;
                }
            }
            match scissor(key1, key2) {
                1 => {
                    stats.hsb += 1;
                    if get_bad_bigrams {
                        bigram_weight += 15;
                    }
                    if command == "hsb" {
                        insert_bigram = true;
                    }
                }
                2 => {
                    stats.fsb += 1;
                    if get_bad_bigrams {
                        bigram_weight += 75;
                    }
                    if command == "fsb" {
                        insert_bigram = true;
                    }
                }
                _ => {}
            }
        }
    }
    (insert_bigram, bigram_weight.try_into().unwrap())
}

pub fn skipgram_stats(
    key1: &Key,
    key2: &Key,
    epic_key1: &Key,
    command: &str,
    stats: &mut Stats,
    finger_weights: &AHashMap<Finger, i64>,
) -> bool {
    let mut insert_ngram = false;
    stats.skipgrams += 1;
    if key1.hand == key2.hand && key1.finger != Finger::Thumb && key2.finger != Finger::Thumb {
        if key1.finger == key2.finger && key1.row != key2.row {
            let dy = key1.row.abs_diff(key2.row);
            let distance = if key1.lateral == key2.lateral {
                dy.max(1)
            } else {
                (dy.pow(2) + 1).isqrt()
            };
            stats.fspeed += distance as i64 * finger_weights[&key1.finger];
            stats.sfs += 1;
            if command == "sfs" {
                insert_ngram = true;
            }
        } else {
            if key1.lateral || key2.lateral {
                stats.lss += 1;
                if command == "lss" {
                    insert_ngram = true;
                }
            }
            match scissor(key1, key2) {
                1 => {
                    stats.hss += 1;
                    if command == "hss" {
                        insert_ngram = true;
                    }
                }
                2 => {
                    stats.fss += 1;
                    if command == "fss" {
                        insert_ngram = true;
                    }
                }
                _ => {}
            }
        }

        if epic_key1.hand == key2.hand {
            stats.skipgrams += 1;
            if epic_key1.finger == key2.finger && epic_key1 != key2 {
                stats.sfs += 1;
                if command == "sfs" {
                    insert_ngram = true;
                }
            }
            if key2.lateral || epic_key1.lateral && epic_key1 != key2 {
                stats.lss += 1;
                if command == "lss" {
                    insert_ngram = true;
                }
            }
            match scissor(key1, key2) {
                1 => {
                    stats.hss += 1;
                    if command == "hss" {
                        insert_ngram = true;
                    }
                }
                2 => {
                    stats.fss += 1;
                    if command == "fss" {
                        insert_ngram = true;
                    }
                }
                _ => {}
            }
        }
    }

    insert_ngram
}
pub fn sf(key1: &Key, key2: &Key) -> bool {
    if key1.finger == key2.finger && key1.hand == key2.hand && key1 != key2 {
        return true;
    }
    false
}

pub fn ls(key1: &Key, key2: &Key) -> bool {
    if (key1.lateral || key2.lateral)
        && key1.hand == key2.hand
        && key1.finger != Finger::Thumb
        && key2.finger != Finger::Thumb
    {
        return true;
    }
    false
}

pub fn scissor(key1: &Key, key2: &Key) -> u8 {
    let distance: u8 = (i64::from(key1.row) - i64::from(key2.row))
        .abs()
        .try_into()
        .expect("invalid distance");
    if key1.hand == key2.hand
        && key1.finger != key2.finger
        && (((key1.finger == Finger::Pinky || key1.finger == Finger::Index)
            && (key2.finger == Finger::Middle || key2.finger == Finger::Ring))
            || ((key2.finger == Finger::Pinky || key2.finger == Finger::Index)
                && (key1.finger == Finger::Middle || key1.finger == Finger::Ring))
            || (key1.finger == Finger::Middle && key2.finger == Finger::Ring) || (key2.finger == Finger::Middle && key1.finger == Finger::Ring))
    {
        return distance;
    }
    0
}
