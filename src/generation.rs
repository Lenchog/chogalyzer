use indicatif::ProgressBar;
use indicatif::MultiProgress;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::thread;
use crate::{stats, Stats};
const THREADS: usize = 8;

pub fn generate_threads(layout_raw: [char; 32], corpus: &String, max_iterations: u64) -> ([char; 32], i32, Vec<String>) {
    let mut layouts: [([char; 32], i32, Vec<String>); THREADS] = Default::default();
    let bars = MultiProgress::new();
    thread::scope(|s| {
      let vec: Vec<_> = (0..THREADS).map(|_| s.spawn(|| generate(layout_raw, corpus, max_iterations, &bars) )).collect();
      for (i, handle) in vec.into_iter().enumerate() {
        let res = handle.join();
        layouts[i] = Result::expect(res, "idk"); 
      }
    });
    compare_layouts(layouts)
}

fn generate(layout_raw: [char; 32], corpus: &String, max_iterations: u64, multibars: &MultiProgress) -> ([char; 32], i32, Vec<String>) {
    let mut rng = thread_rng();
    let mut layout: ([char; 32], Stats, Vec<String>) = (layout_raw, Stats::default(), vec!("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()));
    layout.0.shuffle(&mut rng);
    layout.1 = stats::analyze(corpus.to_string(), layout.0, &"generate".to_string(), layout.2.clone().to_vec());
    let bar = ProgressBar::new(max_iterations);
    multibars.add(bar.clone());

    let mut stat_array: [Stats; 10] = Default::default();
    for layout_stats in &mut stat_array {
        let mut rng = rand::thread_rng();
        let letter1 = rng.gen_range(0..layout.0.len());
        let letter2 = rng.gen_range(0..layout.0.len());
        layout.0.swap(letter1, letter2);
        layout.1 = stats::analyze(corpus.to_string(), layout.0, &"generate".to_string(), layout.2.clone().to_vec());
        *layout_stats = layout.1.clone();
    }
    let mut iterations = 0;
    while iterations < max_iterations {
        iterations += 1;
        layout = attempt_swap(false, layout.0, layout.2, corpus, layout.1.clone(), layout.1.bad_bigrams);
        layout = attempt_swap(true, layout.0, layout.2, corpus, layout.1.clone(), layout.1.bad_bigrams);
        bar.inc(1);
    }
    (layout.0, layout.1.score, layout.2)
}

fn attempt_swap(
    do_magic: bool,
    layout: [char; 32],
    magic: Vec<String>,
    corpus: &String,
    old_stats: Stats,
    bad_bigrams: Vec<String>,
) -> ([char; 32], Stats, Vec<String>) {
    let mut rng = rand::thread_rng();
    let letter1 = rng.gen_range(0..layout.len());
    let letter2 = rng.gen_range(0..layout.len());

    let mut new_layout = layout;
    let mut new_magic = magic.clone();
    if do_magic {
        new_magic = swap_magic(new_magic, bad_bigrams);
    }
    else {
        new_layout.swap(letter1, letter2);
    }

    let new_stats = stats::analyze(
        corpus.to_string(),
        new_layout,
        &"generate".to_string(),
        new_magic.clone().to_vec()
    );

    
    if new_stats.score > old_stats.score {
        (new_layout, new_stats, new_magic)
    } else {
        (layout, old_stats, magic)
    }
}

fn compare_layouts(layouts: [([char; 32], i32, Vec<String>); THREADS]) -> ([char; 32], i32, Vec<String>) {
    let mut best_score = layouts[0].1;
    let mut best_layout = 0;
    for (i, item) in layouts.iter().enumerate() {
        if item.1 > best_score {
            best_layout = i;
            best_score = item.1;
        }
    }
    layouts[best_layout].clone()
}

fn swap_magic(mut magic_rules: Vec<String>, bad_bigrams: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    let random_rule = bad_bigrams.choose(&mut rng).unwrap().to_string();
    for rule in &magic_rules {
        if !rule.is_empty() && random_rule.chars().next().unwrap() == rule.chars().next().unwrap() {
            return swap_magic(magic_rules, bad_bigrams);
        }
    }
    let random_pos: usize = rng.gen_range(0..magic_rules.len());
    magic_rules[random_pos] = random_rule;
    magic_rules
}
