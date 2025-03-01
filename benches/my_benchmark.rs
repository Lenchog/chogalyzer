use chogalyzer::generation::attempt_swap;
use chogalyzer::*;
use diol::prelude::*;
use std::env::args;
use std::fs;
use std::mem::swap;

fn main() -> std::io::Result<()> {
    let mut bench = Bench::new(BenchConfig::from_args()?);
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

    let corpus: String = fs::read_to_string("mr.txt")
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
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));
    bench.register(swap_letters, layout);
    bench.register(swap_letters, (layout));
    bench.run()?;
    Ok(())
}

fn swap_letters(bencher: Bencher, corpus: &String) {
    layout = "abcdefghijklmnopqrstuv,.';*⎵".shuffle();
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
}
