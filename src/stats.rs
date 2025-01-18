mod trigram_stats;
mod bigram_stats;

use crate::{Key, Stats, INCLUDE_SPACE, INCLUDE_THUMB_ALT, INCLUDE_THUMB_ROLL};
use std::collections::HashMap;

pub fn analyze(corpus: String, layout: &HashMap<char, Key>, command: &String) -> Stats {
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
