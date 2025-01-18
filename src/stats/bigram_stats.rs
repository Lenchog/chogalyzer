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
    if sfr(key1, key2) {
        stats.sfr += 1;
        if command == "sfr" {
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
