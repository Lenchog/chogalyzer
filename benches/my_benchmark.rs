use ahash::AHashMap;
use chogalyzer::stats::bigram_stats::{bigram_stats, scissor, skipgram_stats};
use chogalyzer::stats::layout_raw_to_table;
use chogalyzer::stats::trigram_stats::trigram_stat;
use chogalyzer::*;
use diol::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut bench = Bench::new(BenchConfig::from_args()?);

    bench.register(bench_bigram_stats, ["wh", "tm", "th", "el", "y."]);
    bench.register(bench_scissor, ["el", "l.", "th"]);
    bench.register(bench_skipgram_stats, ["whn", "tmn", "thn", "elr", "y.r"]);
    bench.register(bench_trigram_stats, ["you", "thr", "ale", "atr"]);
    bench.run()?;
    Ok(())
}

fn bench_bigram_stats(bencher: Bencher, letters: &str) {
    let command = &String::from("bench");
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);
    let mut stats = Stats::default();
    let layout_letters: String = fs::read_to_string("whirl.txt")
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .replace('_', "⎵")
        .chars()
        .collect();

    // has to be 37 because ⎵ is a few extra bytes
    let layout_raw: [char; 32] = layout_letters[..37]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout");

    let table = layout_raw_to_table(&layout_raw);
    let key1 = &table[&letters.chars().next().unwrap()];
    let key2 = &table[&letters.chars().nth(1).unwrap()];
    bencher.bench(|| {
        bigram_stats(&key1, &key2, command, &mut stats, &finger_weights, false);
    })
}
fn bench_scissor(bencher: Bencher, letters: &str) {
    let layout_letters: String = fs::read_to_string("whirl.txt")
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .replace('_', "⎵")
        .chars()
        .collect();

    // has to be 37 because ⎵ is a few extra bytes
    let layout_raw: [char; 32] = layout_letters[..37]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout");

    let table = layout_raw_to_table(&layout_raw);
    let key1 = &table[&letters.chars().next().unwrap()];
    let key2 = &table[&letters.chars().nth(1).unwrap()];
    bencher.bench(|| {
        scissor(&key1, &key2);
    })
}

fn bench_skipgram_stats(bencher: Bencher, letters: &str) {
    let command = &String::from("bench");
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);
    let mut stats = Stats::default();
    let layout_letters: String = fs::read_to_string("whirl.txt")
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .replace('_', "⎵")
        .chars()
        .collect();

    // has to be 37 because ⎵ is a few extra bytes
    let layout_raw: [char; 32] = layout_letters[..37]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout");

    let table = layout_raw_to_table(&layout_raw);
    let key1 = &table[&letters.chars().next().unwrap()];
    let key2 = &table[&letters.chars().nth(1).unwrap()];
    let epickey = &table[&letters.chars().nth(2).unwrap()];
    bencher.bench(|| {
        skipgram_stats(&key1, &key2, &epickey, command, &mut stats, &finger_weights);
    })
}

fn bench_trigram_stats(bencher: Bencher, letters: &str) {
    let layout_letters: String = fs::read_to_string("whirl.txt")
        .expect("couldn't read layout")
        .replace([' ', ' '], "")
        .replace('_', "⎵")
        .chars()
        .collect();

    // has to be 37 because ⎵ is a few extra bytes
    let layout_raw: [char; 32] = layout_letters[..37]
        .replace('\n', "")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("couldn't read layout");

    let table = layout_raw_to_table(&layout_raw);
    let key1 = &table[&letters.chars().next().unwrap()];
    let key2 = &table[&letters.chars().nth(1).unwrap()];
    let key3 = &table[&letters.chars().nth(2).unwrap()];
    bencher.bench(|| {
        trigram_stat(&key1, &key2, &key3);
    })
}

/* fn swap_letters(bencher: Bencher, corpus: &String) {
    let layout = "abcdefghijklmnopqrstuv,.';*⎵".shuffle();
    old_stats = attempt_swap(
        do_magic,
        layout,
        magic,
        corpus,
        old_stats,
        bad_bigrams,
        temparature,
    );
    bencher.bench(|| {
        attempt_swap(
            false,
            layout,
            Vec::default(),
            corpus,
            old_stats,
            bad_bigrams,
            temparature,
        )
    })
} */
