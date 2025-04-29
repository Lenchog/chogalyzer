use crate::{
    stats::{self, analyze, bigram_stats, layout_raw_to_table},
    Algorithm, Finger, Layout, Stats,
};
use ahash::{AHashMap, AHashSet};
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::{thread, time::Instant};
const THREADS: usize = 8;

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
            .map(|_| {
                s.spawn(|| {
                    generate(
                        layout_raw,
                        corpus,
                        max_iterations,
                        &bars,
                        magic_rules,
                        cooling_rate,
                        algorithm.clone(),
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
    algorithm: Algorithm,
) -> Layout {
    let mut layout = randomise_layout(layout_raw, corpus.clone(), magic_rules);
    let mut iterations = 0;
    //let bar = ProgressBar::new(max_iterations);
    //multibars.add(bar.clone());
    // specifically for sim annealing
    let mut temperature = get_temperature(&mut layout, &corpus);

    let start = Instant::now();
    while iterations < max_iterations {
        iterations += 1;
        let new_layout = if algorithm == Algorithm::HillClimbing {
            find_best_swap(layout.layout, corpus, magic_rules)
        } else if algorithm == Algorithm::RandomLayout {
            randomise_layout(layout_raw, corpus.clone(), magic_rules)
        } else {
            attempt_swap(layout.clone(), corpus, magic_rules)
        };
        layout = if algorithm == Algorithm::HillClimbing
            || (algorithm == Algorithm::SimAnnealing
                && annealing_assess(&layout.stats, &new_layout.stats, temperature))
            || ((algorithm == Algorithm::GreedySwapping || algorithm == Algorithm::RandomLayout)
                && new_layout.stats.score > layout.stats.score)
        {
            //println!("Layout\n{}\n{}\n{}\n      {}    {}\n", layout.layout[0..9], layout.layout[10..19], layout.layout[20..29], layout.layout[30], layout.layout[31]);
            println!("{}, {}", start.elapsed().as_millis(), layout.stats.score);
            new_layout.clone()
        } else {
            if algorithm == Algorithm::HillClimbing {
                return layout;
            }
            layout
        };
        //bar.inc(1);
        temperature *= cooling_rate;
    }
    layout
}

fn randomise_layout(layout_raw: [char; 32], corpus: String, magic_rule_number: usize) -> Layout {
    let mut rng = thread_rng();
    let mut new_layout_raw = layout_raw;
    new_layout_raw.shuffle(&mut rng);
    let magic_rules = get_magic_rules(&corpus, new_layout_raw, magic_rule_number);
    let stats = analyze(corpus, new_layout_raw, "generate", &magic_rules);
    Layout {
        layout: new_layout_raw,
        magic: magic_rules,
        stats: stats,
    }
}

fn find_best_swap(layout_raw: [char; 32], corpus: &String, magic_rules_number: usize) -> Layout {
    let old_layout = layout_raw;
    let old_magic = get_magic_rules(corpus, layout_raw, magic_rules_number);
    let old_stats = analyze(corpus.to_string(), layout_raw, "generate", &old_magic);
    let mut best_layout = Layout {
        layout: old_layout,
        magic: old_magic,
        stats: old_stats,
    };
    for letter1 in 0..layout_raw.len() {
        for letter2 in 0..layout_raw.len() {
            let mut new_layout = old_layout.clone();
            new_layout.swap(letter1, letter2);
            let new_magic_rules = get_magic_rules(corpus, new_layout, magic_rules_number);
            let new_stats = analyze(
                corpus.to_string(),
                new_layout,
                "generate",
                &new_magic_rules,
            );
            if new_stats.score > best_layout.stats.score {
                best_layout = Layout {
                    layout: new_layout,
                    stats: new_stats,
                    magic: new_magic_rules,
                };
            }
        }
    }
    best_layout
}

fn get_temperature(layout: &mut Layout, corpus: &String) -> f64 {
    let stat_array: &[Stats; 10] = &Default::default();
    for ref mut layout_stats in stat_array {
        let mut rng = rand::thread_rng();
        let letter1 = rng.gen_range(0..layout.layout.len());
        let letter2 = rng.gen_range(0..layout.layout.len());
        layout.layout.swap(letter1, letter2);
        layout.stats = stats::analyze(corpus.to_string(), layout.layout, "generate", &layout.magic);
        *layout_stats = &layout.stats.clone();
    }
    standard_deviation(&stat_array.clone())
}

pub fn attempt_swap(old_layout: Layout, corpus: &String, magic_rules: usize) -> Layout {
    let mut rng = rand::thread_rng();
    let mut new_layout = old_layout;
    // swap letters or column
    if rng.gen_range(0..10) > 3 {
        new_layout
            .layout
            .swap(rng.gen_range(0..32), rng.gen_range(0..32));
    } else {
        new_layout.layout = column_swap(
            new_layout.layout,
            rng.gen_range(1..10),
            rng.gen_range(1..10),
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

pub fn annealing_assess(old_stats: &Stats, new_stats: &Stats, temperature: f64) -> bool {
    new_stats.score > old_stats.score
        || annealing_func(old_stats.score, new_stats.score, temperature)
}

fn compare_layouts(layouts: &[Layout; THREADS]) -> Layout {
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
fn annealing_func(old: f64, new: f64, temperature: f64) -> bool {
    let mut rng = ThreadRng::default();
    let delta: f64 = new - old;
    let probability = 1.0 / (1.0 + (delta / temperature).exp());
    rng.gen_range(0.0..1.0) > probability
}

fn column_swap(mut layout: [char; 32], col1: usize, col2: usize) -> [char; 32] {
    layout.swap(col1, col2);
    layout.swap(col1 + 10, col2 + 10);
    layout.swap(col1 + 20, col2 + 20);
    layout
}

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
    sorted_vec.sort_by(|a, b| b.1.cmp(&a.1));

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
