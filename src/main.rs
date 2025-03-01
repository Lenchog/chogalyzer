use clap::Parser;
use std::fs;
use std::cmp;

use chogalyzer::{generation, output, stats};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "whirl.txt")]
    layout: String,

    #[arg(short, long, default_value = "mr.txt")]
    corpus: String,

    #[arg(default_value = "analyze")]
    command: String,

    #[arg(short, long, default_value_t = 500)]
    iterations: u64,

    #[arg(short, long, default_value_t = 10)]
    magic_rules: usize,

    #[arg(long, default_value_t = 0.99)]
    cooling: f64,
}

fn main() {
    let args = Args::parse();

    let layout_letters: String = fs::read_to_string(args.layout.clone())
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

let corpus: String = fs::read_to_string(&args.corpus)
    .expect("error reading corpus")
    .replace("\n\n", "")
    .replace(' ', "⎵")
    .chars()
    .map(|ch| {
        if ch.is_ascii_uppercase() {
            // Replace uppercase letters with "*" followed by lowercase
            format!("*{}", ch.to_ascii_lowercase()).chars().collect::<Vec<_>>()
        } else {
            // Leave other characters unchanged
            vec![ch]
        }
    })
    .flatten()
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
        &args.command,
        &magic_rules,
    );

    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => output::print_stats(
            &stats,
            layout_raw,
            &magic_rules,
            args.layout.clone().strip_suffix(".txt").unwrap(),
        ),
        "generate" => {
            let layout = generation::generate_threads(
                layout_raw,
                &corpus,
                args.iterations,
                args.magic_rules,
                args.cooling,
            );
            output::print_stats(
                &stats::analyze(corpus.clone(), layout.0, &args.command, &layout.2),
                layout.0,
                &layout.2,
                layout.0[10..15].iter().collect::<String>().as_str(),
            );
            /* for magic_rules in 1..20 {
                let layout = generation::generate_threads(layout_raw, &corpus, args.iterations, magic_rules as usize, args.cooling);
                println!("{} Magic rules: {}", magic_rules, layout.1);
                //output::print_stats(stats::analyze(corpus.clone(), layout.0, &args.command, layout.2.clone()), layout.0, &layout.2);
            } */
        }
        "sfb" => output::print_ngrams(&ngram_vec, stats.bigrams, "SFB".to_string()),
        "sfr" => output::print_ngrams(&ngram_vec, stats.bigrams, "SFR".to_string()),
        "sfs" => output::print_ngrams(&ngram_vec, stats.skipgrams, "SFS".to_string()),
        "lsbs" => output::print_ngrams(&ngram_vec, stats.bigrams, "LSB".to_string()),
        "lss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "LSS".to_string()),
        "fsb" => output::print_ngrams(&ngram_vec, stats.bigrams, "FSB".to_string()),
        "fss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "FSS".to_string()),
        "alt" => output::print_ngrams(&ngram_vec, stats.trigrams, "Alt".to_string()),
        "inroll" => output::print_ngrams(&ngram_vec, stats.trigrams, "Inroll".to_string()),
        "outroll" => output::print_ngrams(&ngram_vec, stats.trigrams, "Outroll".to_string()),
        "inthreeroll" => {
            output::print_ngrams(&ngram_vec, stats.trigrams, "Inthreeroll".to_string());
        }
        "outthreeroll" => {
            output::print_ngrams(&ngram_vec, stats.trigrams, "Outthreeroll".to_string());
        }
        "red" => output::print_ngrams(&ngram_vec, stats.trigrams, "Red".to_string()),
        "weak" => output::print_ngrams(&ngram_vec, stats.trigrams, "Weak".to_string()),
        "thumb" => output::print_ngrams(&ngram_vec, stats.trigrams, "Thumb".to_string()),
        "bigrams" => output::print_ngrams(&ngram_vec, stats.bigrams, "Bigrams".to_string()),
        "skipgrams" => output::print_ngrams(&ngram_vec, stats.skipgrams, "Skipgrams".to_string()),
        "trigrams" => output::print_ngrams(&ngram_vec, stats.trigrams, "Trigrams".to_string()),
        _ => println!("invalid command"),
    }
}
