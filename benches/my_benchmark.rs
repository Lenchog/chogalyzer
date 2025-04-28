use ahash::AHashMap;
use chogalyzer::generation::get_magic_rules;
use chogalyzer::stats::bigram_stats::{bigram_stats, scissor, skipgram_stats};
use chogalyzer::stats::trigram_stats::trigram_stat;
use chogalyzer::stats::{analyze, layout_raw_to_table};
use chogalyzer::*;
use chogalyzer::{load_layout, load_magic_rules};
use diol::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut bench = Bench::new(BenchConfig::from_args()?);

    bench.register(bench_bigram_stats, ["de", "ey", "er", "li", "ex"]);
    bench.register(bench_scissor, ["ex", "li", "er"]);
    bench.register(bench_skipgram_stats, ["eda", "tmn", "thn", "elr", "y.r"]);
    bench.register(bench_trigram_stats, ["you", "thr", "ale", "atr"]);
    bench.register(bench_analyse, ["analyze", "sfb"]);
    bench.register(bench_get_magic_rules, [1, 10, 20]);
    bench.run()?;
    Ok(())
}

fn bench_analyse(bencher: Bencher, command: &str) {
    let magic_rules = load_magic_rules();
    let corpus: String = fs::read_to_string("corpora/filtered/mr.txt").expect("corpus not loaded");
    let layout_raw = load_layout();
    bencher.bench(|| analyze(corpus.clone(), layout_raw, command, &magic_rules));
}

fn bench_get_magic_rules(bencher: Bencher, magic_rules: usize) {
    let corpus: String = fs::read_to_string("corpora/filtered/mr.txt").expect("corpus not loaded");
    let layout_raw = load_layout();
    bencher.bench(|| get_magic_rules(&corpus, layout_raw, magic_rules));
}

fn bench_bigram_stats(bencher: Bencher, letters: &str) {
    let mut stats = Stats::default();
    let finger_weights = load_finger_weights();

    let (key1, key2) = load_two_keys(letters);
    bencher.bench(|| {
        bigram_stats(&key1, &key2, "bench", &mut stats, &finger_weights, false);
    })
}
fn bench_scissor(bencher: Bencher, letters: &str) {
    let (key1, key2) = load_two_keys(letters);
    bencher.bench(|| {
        scissor(&key1, &key2);
    })
}

fn bench_skipgram_stats(bencher: Bencher, letters: &str) {
    let command = &String::from("bench");
    let finger_weights = load_finger_weights();
    let mut stats = Stats::default();

    let (key1, key2, epic_key) = load_three_keys(letters);
    bencher.bench(|| {
        skipgram_stats(
            &key1,
            &key2,
            &epic_key,
            command,
            &mut stats,
            &finger_weights,
        );
    })
}

fn bench_trigram_stats(bencher: Bencher, letters: &str) {
    let (key1, key2, key3) = load_three_keys(letters);
    bencher.bench(|| {
        trigram_stat(&key1, &key2, &key3);
    })
}

fn load_three_keys(letters: &str) -> (Key, Key, Key) {
    let layout_raw = &load_layout();
    let table = layout_raw_to_table(layout_raw);
    let key1 = table[&letters.chars().next().unwrap()].clone();
    let key2 = table[&letters.chars().nth(1).unwrap()].clone();
    let key3 = table[&letters.chars().nth(2).unwrap()].clone();
    (key1, key2, key3)
}

fn load_two_keys(letters: &str) -> (Key, Key) {
    let layout_raw = &load_layout();
    let table = layout_raw_to_table(layout_raw);
    let key1 = table[&letters.chars().next().unwrap()].clone();
    let key2 = table[&letters.chars().nth(1).unwrap()].clone();
    (key1, key2)
}

fn load_finger_weights() -> AHashMap<Finger, i64> {
    AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ])
}
