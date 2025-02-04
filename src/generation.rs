use crate::{stats, Stats};
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::thread;
const THREADS: usize = 8;

#[must_use]
pub fn generate_threads(
    layout_raw: [char; 32],
    corpus: &String,
    max_iterations: u64,
    magic_rules: usize,
    cooling_rate: f64,
) -> ([char; 32], f64, Vec<String>) {
    let mut layouts: [([char; 32], f64, Vec<String>); THREADS] = Default::default();
    let bars = MultiProgress::new();
    thread::scope(|s| {
        let vec: Vec<_> = (0..THREADS)
            .map(|_| {
                s.spawn(|| {
                    generate(
                        layout_raw,
                        corpus,
                        max_iterations,
                        &bars,
                        magic_rules,
                        cooling_rate,
                    )
                })
            })
            .collect();
        for (i, handle) in vec.into_iter().enumerate() {
            let res = handle.join();
            layouts[i] = Result::expect(res, "idk");
        }
    });
    compare_layouts(&layouts)
}

fn generate(
    layout_raw: [char; 32],
    corpus: &String,
    max_iterations: u64,
    multibars: &MultiProgress,
    magic_rules: usize,
    cooling_rate: f64,
) -> ([char; 32], f64, Vec<String>) {
    let mut rng = thread_rng();
    let mut layout: ([char; 32], Stats, Vec<String>) = (
        layout_raw,
        Stats::default(),
        vec![String::default(); magic_rules],
    );
    layout.0.shuffle(&mut rng);
    layout.1 = stats::analyze(
        corpus.to_string(),
        layout.0,
        &"generate".to_string(),
        layout.2.clone().clone(),
    );
    let bar = ProgressBar::new(max_iterations);
    multibars.add(bar.clone());
    let stat_array: &[Stats; 10] = &Default::default();
    for ref mut layout_stats in stat_array {
        let mut rng = rand::thread_rng();
        let letter1 = rng.gen_range(0..layout.0.len());
        let letter2 = rng.gen_range(0..layout.0.len());
        layout.0.swap(letter1, letter2);
        layout.1 = stats::analyze(
            corpus.to_string(),
            layout.0,
            &"generate".to_string(),
            layout.2.clone().clone(),
        );
        *layout_stats = &layout.1.clone();
    }
    let mut temparature = standard_deviation(&stat_array.clone());
    let mut iterations = 0;
    while iterations < max_iterations {
        iterations += 1;
        layout = attempt_swap(
            false,
            layout.0,
            layout.2,
            corpus,
            layout.1.clone(),
            &layout.1.bad_bigrams,
            temparature,
        );
        //dbg!(&layout.1.bad_bigrams);
        if layout.1.bad_bigrams.is_empty() {
            println!("perfection achieved baby!");
        } else {
            layout = attempt_swap(
                true,
                layout.0,
                layout.2,
                corpus,
                layout.1.clone(),
                &layout.1.bad_bigrams,
                temparature,
            );
        }
        bar.inc(1);
        temparature *= cooling_rate;
    }
    (layout.0, layout.1.score, layout.2)
}

fn attempt_swap(
    do_magic: bool,
    layout: [char; 32],
    magic: Vec<String>,
    corpus: &String,
    old_stats: Stats,
    bad_bigrams: &[String],
    temparature: f64,
) -> ([char; 32], Stats, Vec<String>) {
    let mut rng = rand::thread_rng();
    let letter1 = rng.gen_range(0..layout.len());
    let letter2 = rng.gen_range(0..layout.len());

    let mut new_layout = layout;
    let mut new_magic = magic.clone();
    if do_magic {
        new_magic = swap_magic(new_magic, bad_bigrams);
    } else if rng.gen_range(0..10) > 3 {
        new_layout.swap(letter1, letter2);
    } else {
        new_layout = column_swap(new_layout, rng.gen_range(1..10), rng.gen_range(1..10));
    }

    let new_stats = stats::analyze(
        corpus.to_string(),
        new_layout,
        &"generate".to_string(),
        new_magic.clone().clone(),
    );

    if new_stats.score > old_stats.score
        || annealing_func(old_stats.score, new_stats.score, temparature)
    {
        (new_layout, new_stats, new_magic)
    } else {
        (layout, old_stats, magic)
    }
}

fn compare_layouts(
    layouts: &[([char; 32], f64, Vec<String>); THREADS],
) -> ([char; 32], f64, Vec<String>) {
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

fn swap_magic(mut magic_rules: Vec<String>, bad_bigrams: &[String]) -> Vec<String> {
    let mut rng = thread_rng();
    let random_rule = bad_bigrams.choose(&mut rng).unwrap().to_string();
    for ref mut rule in &magic_rules {
        if !rule.is_empty() && random_rule.chars().next().unwrap() == rule.chars().next().unwrap() {
            *rule = &random_rule;
            return magic_rules;
        }
    }
    let random_pos: usize = rng.gen_range(0..magic_rules.len());
    magic_rules[random_pos] = random_rule;
    magic_rules
}
fn standard_deviation(stat_array: &[Stats; 10]) -> f64 {
    let mut sum = 0.0;
    for layout in stat_array {
        sum += layout.score;
    }
    let mean = sum / stat_array.len() as f64;
    sum = 0.0;
    for layout in stat_array {
        sum += (layout.score - mean).powf(2.0);
    }
    let variance = sum / stat_array.len() as f64;
    variance.sqrt()
}
fn annealing_func(old: f64, new: f64, temparature: f64) -> bool {
    let mut rng = ThreadRng::default();
    let delta: f64 = new - old;
    let probability = 1.0 / (1.0 + (delta / temparature).exp());
    rng.gen_range(0.0..1.0) > probability
}

fn column_swap(mut layout: [char; 32], col1: usize, col2: usize) -> [char; 32] {
    layout.swap(col1, col2);
    layout.swap(col1 + 10, col2 + 10);
    layout.swap(col1 + 20, col2 + 20);
    layout
}
