pub mod bigram_stats;
pub mod trigram_stats;

use crate::{Finger, Key, Stats, INCLUDE_THUMB_ALT, INCLUDE_THUMB_ROLL};
use ahash::AHashMap;

#[must_use]
pub fn analyze(
    mut corpus: String,
    layout_letters: [char; 32],
    command: &String,
    magic_rules: &Vec<String>,
) -> Stats {
    let layout = layout_raw_to_table(&layout_letters);
    let [mut previous_letter, mut skip_previous_letter, mut epic_previous_letter] = ['_'; 3];
    let mut stats: Stats = Stats::default();
    let mut char_freq: AHashMap<char, u32> = AHashMap::default();
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);

    for rule in magic_rules {
        if !rule.is_empty() {
            if let Some(v) = rule.chars().next() {
                corpus = corpus.replace(rule, &(v.to_string() + "*"));
            }
        }
    }

    for letter_u8 in corpus.as_bytes() {
        let letter = *letter_u8 as char;
        let key = &layout[&letter];
        let previous_key = &layout[&previous_letter];
        let skip_previous_key = &layout[&skip_previous_letter];
        let epic_previous_key = &layout[&epic_previous_letter];
        let bigram = bigram_stats::bigram_stats(
            previous_key,
            key,
            command,
            &mut stats,
            &finger_weights,
            false,
        );
        *char_freq.entry(letter).or_insert(0) += 1;
        if bigram.0 {
            *stats
                .ngram_table
                .entry([previous_letter, letter, ' '])
                .or_insert(0) += 1;
        }
        if bigram.1 > 0 {
            *stats
                .bad_bigrams
                .entry([previous_letter, letter])
                .or_insert(0) += bigram.1;
        }
        let skipgram = bigram_stats::skipgram_stats(
            skip_previous_key,
            key,
            epic_previous_key,
            command,
            &mut stats,
            &finger_weights,
        );
        if skipgram {
            *stats
                .ngram_table
                .entry([skip_previous_letter, letter, ' '])
                .or_insert(0) += 1;
        }
        let trigram =
            trigram_stats::trigram_stats(skip_previous_key, previous_key, key, command, stats);
        stats = trigram.0;
        stats.trigrams += 1;
        if trigram.1 {
            *stats
                .ngram_table
                .entry([skip_previous_letter, previous_letter, letter])
                .or_insert(0) += 1;
        }
        epic_previous_letter = letter;
        skip_previous_letter = previous_letter;
        previous_letter = letter;
    }
    if !(INCLUDE_THUMB_ALT || INCLUDE_THUMB_ROLL) {
        stats.trigrams -= stats.thumb_stat;
    }
    #[rustfmt::skip]
    let weighting: [u32; 32] = [
        12, 4, 3, 6, 7, 7, 6, 3, 4, 12, 
        3,  1, 0, 0, 6, 6, 0, 0, 1, 3, 
        8,  9, 8, 7, 9, 9, 7, 8, 9, 8, 
                  0,       0,
    ];
    for i in 0..layout_letters.len() {
        if char_freq.contains_key(&layout_letters[i]) {
            stats.heatmap += i64::from(weighting[i] * char_freq[&layout_letters[i]]);
        }
    }
    let weights = Stats {
        score: 0.0,
        heatmap: -200,
        fspeed: -200,
        sfb: 0,
        sfr: 0,
        sfs: 0,
        fsb: -500,
        hsb: -100,
        hss: -20,
        fss: -100,
        lsb: -100,
        lss: -20,
        inroll: 100,
        outroll: 50,
        alt: 0,
        inthreeroll: 300,
        outthreeroll: 150,
        weak_red: -600,
        red: -7,
        thumb_stat: 0,
        bigrams: 0,
        skipgrams: 0,
        trigrams: 0,
        ngram_table: AHashMap::default(),
        bad_bigrams: AHashMap::default(),
    };
    stats.score = score(&stats, &weights);
    stats
}

#[must_use]
pub fn score(stats: &Stats, weighting: &Stats) -> f64 {
    let mut score = 0;
    score += stats.fspeed * weighting.fspeed / 7;
    score += stats.heatmap * weighting.heatmap / 100;
    score += stats.lsb * weighting.lsb;
    score += stats.lss * weighting.lss;
    score += stats.fsb * weighting.fsb;
    score += stats.fss * weighting.fss;
    score += stats.inroll * weighting.inroll;
    score += stats.inthreeroll * weighting.inthreeroll;
    score += stats.outroll * weighting.outroll;
    score += stats.alt * weighting.alt;
    score += stats.outthreeroll * weighting.outthreeroll;
    score += stats.weak_red * weighting.weak_red;
    score += stats.red * weighting.red;
    /* println!("
        score: {}
        heatmap: {}
        fspeed: {}
        lsb: {}
        lss: {}
        fsb: {}
        fss: {}
        inroll: {}
        outroll: {}
        inthreeroll: {}
        outthreeroll: {}
        weak red: {}
        red: {}
    ", score, stats.heatmap * weighting.heatmap / 100, stats.fspeed * weighting.fspeed / 7, stats.lsb * weighting.lsb, stats.lss * weighting.lss, stats.fsb * weighting.fsb, stats.fss * weighting.fss, stats.inroll * weighting.inroll, stats.outroll * weighting.outroll, stats.inthreeroll * weighting.inthreeroll, stats.outthreeroll * weighting.outthreeroll, stats.weak_red * weighting.weak_red, stats.red * weighting.red); */
    score as f64
    //-stats.sfb
}

pub fn layout_raw_to_table(layout_raw: &[char; 32]) -> AHashMap<char, Key> {
    #[rustfmt::skip]
    return AHashMap::from([
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
}
