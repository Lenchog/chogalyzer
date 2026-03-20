use chogalyzer::{
    convert_corpus, generation, load_corpus, load_layout, load_magic_rules,
    output::{self, Display},
    stats, Algorithm, Args,
};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let corpus = load_corpus(&args.corpus, &args.layout);
    let layout_raw = load_layout(&args.layout);
    let magic_rules = load_magic_rules(&args.layout);
    let stats = stats::analyze(corpus.clone(), layout_raw, &args.command, &magic_rules);
    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by_key(|b| std::cmp::Reverse(b.1));

    match args.command.as_str() {
        "analyze" => Display::new(
            args.layout.clone().strip_suffix(".txt").unwrap(),
            layout_raw,
            &stats,
            &magic_rules,
        )
        .full(),
        "generate" => {
            let layout = generation::generate_threads(
                layout_raw,
                &corpus,
                args.iterations,
                args.magic_rules,
                args.cooling,
                chogalyzer::Algorithm::SimAnnealing,
            );
            Display::new(
                // name
                layout.layout[10..15].iter().collect::<String>().as_str(),
                layout.layout,
                &stats::analyze(corpus.clone(), layout.layout, &args.command, &layout.magic),
                &layout.magic,
            )
            .full();
        }
        "get_data" => {
            let algorithms = [
                Algorithm::GreedySwapping,
                Algorithm::SimAnnealing,
                Algorithm::HillClimbing,
                Algorithm::Hybrid,
                Algorithm::RandomLayout,
            ];
            for _ in 0..4 {
                for algorithm in &algorithms {
                    let _ = generation::generate_threads(
                        layout_raw,
                        &corpus,
                        args.iterations,
                        args.magic_rules,
                        args.cooling,
                        algorithm.clone(),
                    );
                }
            }
            println!("done");
        }
        "convert" => convert_corpus(&args.layout, &args.corpus),
        "sfb" => output::print_ngrams(&ngram_vec, stats.chars, "SFB".to_string(), &args),
        "sfr" => output::print_ngrams(&ngram_vec, stats.chars, "SFR".to_string(), &args),
        "sfs" => output::print_ngrams(&ngram_vec, stats.skipgrams, "SFS".to_string(), &args),
        "lsbs" => output::print_ngrams(&ngram_vec, stats.chars, "LSB".to_string(), &args),
        "lss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "LSS".to_string(), &args),
        "fsb" => output::print_ngrams(&ngram_vec, stats.chars, "FSB".to_string(), &args),
        "hsb" => output::print_ngrams(&ngram_vec, stats.chars, "HSB".to_string(), &args),
        "fss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "FSS".to_string(), &args),
        "alt" => output::print_ngrams(&ngram_vec, stats.chars, "Alt".to_string(), &args),
        "inroll" => output::print_ngrams(&ngram_vec, stats.chars, "Inroll".to_string(), &args),
        "outroll" => output::print_ngrams(&ngram_vec, stats.chars, "Outroll".to_string(), &args),
        "inthreeroll" => {
            output::print_ngrams(&ngram_vec, stats.chars, "Inthreeroll".to_string(), &args);
        }
        "outthreeroll" => {
            output::print_ngrams(&ngram_vec, stats.chars, "Outthreeroll".to_string(), &args);
        }
        "red" => output::print_ngrams(&ngram_vec, stats.chars, "Red".to_string(), &args),
        "weak" => output::print_ngrams(&ngram_vec, stats.chars, "Weak".to_string(), &args),
        "thumb" => output::print_ngrams(&ngram_vec, stats.chars, "Thumb".to_string(), &args),
        "bigrams" => output::print_ngrams(&ngram_vec, stats.chars, "Bigrams".to_string(), &args),
        "skipgrams" => {
            output::print_ngrams(&ngram_vec, stats.skipgrams, "Skipgrams".to_string(), &args)
        }
        "trigrams" => output::print_ngrams(&ngram_vec, stats.chars, "Trigrams".to_string(), &args),
        _ => println!("invalid command"),
    }
}
