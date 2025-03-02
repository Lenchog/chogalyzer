use chogalyzer::stats::bigram_stats::{skipgram_stats, bigram_stats, sf, ls, scissor};
use chogalyzer::stats::layout_raw_to_table;
use chogalyzer::stats::trigram_stats::trigram_stat;
use chogalyzer::*;
use diol::prelude::*;
use std::fs;
use ahash::AHashMap;

fn main() -> std::io::Result<()> {
    let mut bench = Bench::new(BenchConfig::from_args()?);

    /* let corpus: String = fs::read_to_string("mr.txt")
        .expect("error reading corpus")
        .to_lowercase()
        .replace(['\n', ' '], "⎵")
        .chars()
        .filter(|ch| layout_raw.contains(ch))
        .collect();

    let magic_rules_raw = layout_letters[38..].split('\n');
    let mut magic_rules: Vec<String> = Vec::default();

    for rule in magic_rules_raw {
        magic_rules.push(rule.to_string());
    }

    let stats = stats::analyze(
        corpus.clone(),
        layout_raw,
        &"bench".to_string(),
        &magic_rules,
    );

    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1)); */
    //bench.register(swap_letters, layout);
    bench.register(bench_bigram_stats, ["wh", "tm", "th", "el", "y."]);
    bench.register(bench_sf, ["wh", "th"]);
    bench.register(bench_ls, ["tm", "th"]);
    bench.register(bench_scissor, ["el", "l.", "th"]);
    bench.register(bench_skipgram_stats, ["whn", "tmn", "thn", "elr", "y.r"]);
    bench.register(bench_trigram_stats, ["you", "thr", "ale", "atr"]);
    bench.run()?;
    Ok(())
}
fn bench_bigram_stats(
    bencher: Bencher,
    letters: &str,
) {
    let command = &String::from("bench");
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);
    let stats = Stats::default();
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
        bigram_stats(&key1, &key2, command, stats.clone(), &finger_weights);
    })
}
fn bench_sf(
    bencher: Bencher,
    letters: &str,
) {
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
        sf(&key1, &key2);
    })
}
fn bench_ls(
    bencher: Bencher,
    letters: &str,
) {
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
        ls(&key1, &key2);
    })
}
fn bench_scissor(
    bencher: Bencher,
    letters: &str,
) {
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
fn bench_skipgram_stats(
    bencher: Bencher,
    letters: &str,
) {
    let command = &String::from("bench");
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);
    let stats = Stats::default();
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
        skipgram_stats(&key1, &key2, &epickey, command, stats.clone(), &finger_weights);
    })
}

fn bench_trigram_stats(
    bencher: Bencher,
    letters: &str,
) {
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
