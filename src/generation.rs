// This should be multiple files I think. Pretty disgusting overall

use crate::{
    stats::{self, analyze, bigram_stats, layout_raw_to_table},
    Algorithm, Finger, Layout, Stats,
};
use ahash::{AHashMap, AHashSet};
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::{fs::OpenOptions, io::Write, thread, time::Instant};

const THREADS: usize = 12;

/// Generates multiple layouts with threads and compares them
// Eww... AI did most of this, I don't really know how it works
// Multithreading is disgusting. Hopefully I can improve this in the future
#[must_use]
pub fn generate_threads(
    layout_raw: [char; 32],
    corpus: &String,
    max_iterations: u64,
    magic_rules: usize,
    cooling_rate: f64,
    algorithm: Algorithm,
) -> Layout {
    let mut layouts: [Layout; THREADS] = Default::default();
    let bars = MultiProgress::new();
    thread::scope(|s| {
        let vec: Vec<_> = (0..THREADS)
            .map(|runid| {
                s.spawn({
                    let bars = bars.clone();
                    let algorithm = algorithm.clone();
                    move || {
                        generate(
                            layout_raw,
                            corpus,
                            max_iterations,
                            &bars,
                            magic_rules,
                            cooling_rate,
                            algorithm,
                            runid,
                        )
                    }
                })
            })
            .collect();
        for (i, handle) in vec.into_iter().enumerate() {
            let res = handle.join();
            layouts[i] = res.unwrap();
        }
    });
    get_best_layout(&layouts)
}

/// Generates one layout, supporting multiple algorithms
// This is pretty disgusting. Oh well.
fn generate(
    layout_raw: [char; 32],
    corpus: &String,
    max_iterations: u64,
    multibars: &MultiProgress,
    magic_rules: usize,
    cooling_rate: f64,
    algorithm: Algorithm,
    runid: usize,
) -> Layout {
    let mut layout = randomise_layout(layout_raw, corpus.clone(), magic_rules);
    let mut iterations = 0;
    let bar = ProgressBar::new(max_iterations);
    multibars.add(bar.clone());
    // specifically for sim annealing
    let mut temperature = get_temperature(&mut layout, corpus);
    let start = Instant::now();
    let hill_switch_temp = 10.0;
    while iterations < max_iterations {
        iterations += 1;
        let new_layout = if algorithm == Algorithm::HillClimbing
            || (algorithm == Algorithm::Hybrid && temperature <= hill_switch_temp)
        {
            let find_best_swap = find_best_swap(layout.layout, corpus, magic_rules);
            if find_best_swap.1 {
                return find_best_swap.0;
            }
            find_best_swap.0
        } else if algorithm == Algorithm::RandomLayout {
            randomise_layout(layout_raw, corpus.clone(), magic_rules)
        } else {
            attempt_swap(layout.clone(), corpus, magic_rules)
        };
        layout = if algorithm == Algorithm::HillClimbing
            || (algorithm == Algorithm::SimAnnealing
                && annealing_func(layout.stats.score, new_layout.stats.score, temperature))
            || ((algorithm == Algorithm::GreedySwapping || algorithm == Algorithm::RandomLayout)
                && new_layout.stats.score > layout.stats.score)
            || (algorithm == Algorithm::Hybrid && temperature <= hill_switch_temp)
        {
            let mut data_file = OpenOptions::new()
                .append(true)
                .open("data.txt")
                .expect("cannot open file");
            data_file
                .write_all(
                    format!(
                        "{} {} {} {}\n",
                        algorithm,
                        runid,
                        start.elapsed().as_millis(),
                        layout.stats.score
                    )
                    .as_bytes(),
                )
                .expect("write failed");
            /* if new_layout.stats.score >= required_score {
                //println!("1 {} {}", algorithm, start.elapsed().as_millis());
                return new_layout;
            }; */
            new_layout.clone()
        } else {
            if algorithm == Algorithm::HillClimbing {
                return layout;
            }
            layout
        };
        bar.inc(1);
        temperature *= cooling_rate;
    }
    //println!("0 {} {}", algorithm, layout.stats.score);
    layout
}

/// Creates a random layout to start generating a layout from
fn randomise_layout(layout_raw: [char; 32], corpus: String, magic_rule_number: usize) -> Layout {
    let mut rng = rand::rng();
    let mut new_layout_raw = layout_raw;
    new_layout_raw.shuffle(&mut rng);
    let magic_rules = get_magic_rules(&corpus, new_layout_raw, magic_rule_number);
    let stats = analyze(corpus, new_layout_raw, "generate", &magic_rules);
    Layout {
        layout: new_layout_raw,
        magic: magic_rules,
        stats,
    }
}

/// For the hill climbing algorithm. Finds the best swap possible
fn find_best_swap(
    layout_raw: [char; 32],
    corpus: &String,
    magic_rules_number: usize,
) -> (Layout, bool) {
    let old_layout = layout_raw;
    let old_magic = get_magic_rules(corpus, layout_raw, magic_rules_number);
    let old_stats = analyze(corpus.to_string(), layout_raw, "generate", &old_magic);
    let mut best_layout = Layout {
        layout: old_layout,
        magic: old_magic,
        stats: old_stats,
    };
    let mut has_changed = false;
    for (letter1, _) in layout_raw.iter().enumerate() {
        for letter2 in (letter1 + 1)..layout_raw.len() {
            let mut new_layout = old_layout;
            new_layout.swap(letter1, letter2);
            let new_magic_rules = get_magic_rules(corpus, new_layout, magic_rules_number);
            let new_stats = analyze(corpus.to_string(), new_layout, "generate", &new_magic_rules);
            if new_stats.score > best_layout.stats.score {
                has_changed = true;
                best_layout = Layout {
                    layout: new_layout,
                    stats: new_stats,
                    magic: new_magic_rules,
                };
            }
        }
    }
    if !has_changed {
        return (best_layout, true);
    };
    (best_layout, false)
}

/// Get the temperature to start out from with the simulated annealing
fn get_temperature(layout: &mut Layout, corpus: &String) -> f64 {
    let mut score_array: [f64; 10] = Default::default();
    for score in &mut score_array {
        let mut rng = rand::rng();
        let letter1 = rng.random_range(0..layout.layout.len());
        let letter2 = rng.random_range(0..layout.layout.len());
        layout.layout.swap(letter1, letter2);
        layout.stats = stats::analyze(corpus.to_string(), layout.layout, "generate", &layout.magic);
        *score = layout.stats.score;
    }
    standard_deviation(&score_array.clone())
}

/// Do a swap and analyse it.
pub fn attempt_swap(old_layout: Layout, corpus: &String, magic_rules: usize) -> Layout {
    let mut rng = rand::rng();
    let mut new_layout = old_layout;
    // swap letters or column
    if rng.random_range(0..10) > 3 {
        new_layout
            .layout
            .swap(rng.random_range(0..32), rng.random_range(0..32));
    } else {
        new_layout.layout = column_swap(
            new_layout.layout,
            rng.random_range(1..10),
            rng.random_range(1..10),
        );
    }

    new_layout.magic = get_magic_rules(&corpus.to_string(), new_layout.layout, magic_rules);

    new_layout.stats = stats::analyze(
        corpus.to_string(),
        new_layout.layout,
        "generate",
        &new_layout.magic,
    );
    new_layout
}

/// Out of all the layouts generated by different threads; find the best one
fn get_best_layout(layouts: &[Layout; THREADS]) -> Layout {
    let mut best_score = layouts[0].stats.score;
    let mut best_layout = 0;
    for (i, layout) in layouts.iter().enumerate() {
        if layout.stats.score > best_score {
            best_layout = i;
            best_score = layout.stats.score;
        }
    }
    layouts[best_layout].clone()
}

/// For simulated annealing. Gets the standard deviation from 10 scores
fn standard_deviation(score_array: &[f64; 10]) -> f64 {
    let mut sum = 0.0;
    for score in score_array {
        sum += score;
    }
    let mean = sum / score_array.len() as f64;
    sum = 0.0;
    for score in score_array {
        sum += (score - mean).powf(2.0);
    }
    let variance = sum / score_array.len() as f64;
    variance.sqrt()
}

/// Return the new one if it's either better, or sim annealing is close enough
fn annealing_func(old: f64, new: f64, temperature: f64) -> bool {
    let mut rng = ThreadRng::default();
    let delta: f64 = new - old;
    let probability = 1.0 / (1.0 + (delta / temperature).exp());
    rng.random_range(0.0..1.0) > probability || new > old
}

/// Swaps two columns on a layout
fn column_swap(mut layout: [char; 32], col1: usize, col2: usize) -> [char; 32] {
    layout.swap(col1, col2);
    layout.swap(col1 + 10, col2 + 10);
    layout.swap(col1 + 20, col2 + 20);
    layout
}

/// Generate magic rules
pub fn get_magic_rules(
    corpus: &str,
    layout_letters: [char; 32],
    magic_rules: usize,
) -> AHashMap<char, char> {
    let layout = layout_raw_to_table(&layout_letters);
    let mut previous_letter = '_';
    let mut stats: Stats = Stats::default();
    let finger_weights: AHashMap<Finger, i64> = AHashMap::from([
        (Finger::Pinky, 66),
        (Finger::Ring, 28),
        (Finger::Middle, 21),
        (Finger::Index, 18),
        (Finger::Thumb, 50),
    ]);

    for letter in corpus.chars() {
        let key = &layout[&letter];
        let previous_key = &layout[&previous_letter];
        let bigram = bigram_stats::bigram_stats(
            previous_key,
            key,
            "get_bad_bigrams",
            &mut stats,
            &finger_weights,
            true,
        );
        if bigram.1 > 0 {
            *stats
                .bad_bigrams
                .entry([previous_letter, letter])
                .or_insert(0) += bigram.1;
        }
        previous_letter = letter;
    }
    let mut sorted_vec: Vec<([char; 2], u32)> = stats.bad_bigrams.into_iter().collect();

    // Sort in descending order based on frequency
    sorted_vec.sort_by_key(|b| std::cmp::Reverse(b.1));

    let mut used_first_letters: AHashSet<char> = AHashSet::new();
    let mut sorted_keys: AHashMap<char, char> = AHashMap::default();

    // Iterate and select only unique first-letter bigrams
    for (key, _) in sorted_vec {
        if !used_first_letters.contains(&key[0])
        /*  && key[0] != key[1] */
        {
            sorted_keys.insert(key[0], key[1]);
            used_first_letters.insert(key[0]); // Mark the first letter as used
        }
        if sorted_keys.len() == magic_rules {
            break;
        }
    }
    sorted_keys
}
