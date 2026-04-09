use ahash::AHashMap;

use crate::Finger;
use crate::Key;
use crate::Stats;

/// Every state a bigram can be
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Bigram {
    None,
    SFB,
    SFR,
    FSB,
    HSB,
    LSB,
    FSLSB,
    HSLSB,
}

pub fn bigram_stats(
    key1: &Key,
    key2: &Key,
    command: &str,
    stats: &mut Stats,
    finger_weights: &AHashMap<Finger, i64>,
) -> (bool, i64) {
    let stat = bigram_stat(key1, key2);
    // If the command is the stat, we return "true" for inserting the bigram.
    // We also return a weight
    match stat {
        Bigram::SFB => {
            stats.sfb += 1;
            let distance_y = key1.row.abs_diff(key2.row);
            // If they're either both lateral or both not lateral,
            // we don't need to do pythag for distance
            let distance: u8 = if key1.lateral == key2.lateral {
                distance_y
            // Otherwise, we do need to do pythag
            } else {
                (distance_y.pow(2) + 1).isqrt()
            };
            let penalty = 5 * finger_weights[&key1.finger] * distance as i64;
            stats.fspeed += penalty;
            (command == "sfb", 5 * penalty)
        }
        Bigram::SFR => {
            stats.sfr += 1;
            let penalty = 2 * finger_weights[&key1.finger];
            stats.fspeed += penalty;
            (command == "sfr", penalty)
        }
        Bigram::FSB => {
            stats.fsb += 1;
            (command == "fsb", 75)
        }
        Bigram::HSB => {
            stats.hsb += 1;
            (command == "hsb", 15)
        }
        Bigram::LSB => {
            stats.lsb += 1;
            (command == "lsb", 15)
        }
        Bigram::FSLSB => {
            stats.fsb += 1;
            stats.lsb += 1;
            (command == "lsb" || command == "fsb", 90)
        }
        Bigram::HSLSB => {
            stats.hsb += 1;
            stats.lsb += 1;
            (command == "lsb" || command == "hsb", 30)
        }
        Bigram::None => (false, 0),
    }
}

/// Get bigram stats
pub fn bigram_stat(key1: &Key, key2: &Key) -> Bigram {
    // If they're the same hand and neither is thumb.
    if key1.hand == key2.hand && key1.finger != Finger::Thumb && key2.finger != Finger::Thumb {
        // Either SFB or SFR
        if key1.finger == key2.finger {
            // are the keys the same?
            let sfr = key1 == key2;
            if sfr {
                return Bigram::SFR;
            } else {
                return Bigram::SFB;
            }
        } else {
            // We can't return immediately in case it's multiple stats
            let lsb = ls(key1, key2);
            let scissor = scissor(key1, key2);
            return if scissor == 1 {
                if lsb {
                    Bigram::HSLSB
                } else {
                    Bigram::HSB
                }
            } else if scissor == 2 {
                if lsb {
                    Bigram::FSLSB
                } else {
                    Bigram::FSB
                }
            } else if lsb {
                return Bigram::LSB;
            } else {
                return Bigram::None;
            };
        };
    } else {
        return Bigram::None;
    }
}

/// Get skipgram stats
// Eww repeated code TODO
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

/// Check if bigram is on the same finger, and not a repeat
pub fn sf(key1: &Key, key2: &Key) -> bool {
    key1.finger == key2.finger && key1.hand == key2.hand && key1 != key2
}

/// Check whether bigram is a lateral stretch
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

/// Check the intensity of a scissor.
/// 0 => not a scissor, 1 => Half Scissor, 2 => Full Scissor
// TODO maybe turn that into an enum
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
            || (key1.finger == Finger::Middle && key2.finger == Finger::Ring)
            || (key2.finger == Finger::Middle && key1.finger == Finger::Ring))
    {
        return distance;
    }
    0
}
