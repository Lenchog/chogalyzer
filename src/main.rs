use ahash::HashMap;
use std::{
    fs::{self, File},
    io::Write,
};

use chogalyzer::{generation, load_layout, load_magic_rules, output, stats, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let corpus = load_corpus(&args.corpus, &args.layout);
    let layout_raw = load_layout(&args.layout);
    let magic_rules = load_magic_rules(&args.layout);
    let stats = stats::analyze(corpus.clone(), layout_raw, &args.command, &magic_rules);
    let mut ngram_vec: Vec<([char; 3], u32)> = stats.ngram_table.clone().into_iter().collect();
    ngram_vec.sort_by(|a, b| b.1.cmp(&a.1));

    match args.command.as_str() {
        "analyze" => output::print_stats(
            &stats,
            layout_raw,
            &magic_rules,
            &args.layout.clone().strip_suffix(".txt").unwrap(),
        ),
        "generate" => {
            let layout = generation::generate_threads(
                layout_raw,
                &corpus,
                args.iterations,
                args.magic_rules,
                args.cooling,
                chogalyzer::Algorithm::GreedySwapping,
            );
            output::print_stats(
                &stats::analyze(corpus.clone(), layout.layout, &args.command, &layout.magic),
                layout.layout,
                &layout.magic,
                layout.layout[10..15].iter().collect::<String>().as_str(),
            );
            /* for _ in 0..4{
            for magic_rules in 5..10 {
                let layout = generation::generate_threads(layout_raw, &corpus, args.iterations, magic_rules as usize, args.cooling);
                println!("{}, {}", magic_rules, layout.1);
                //output::print_stats(stats::analyze(corpus.clone(), layout.layout, &args.command, layout.magic.clone()), layout.0, &layout.2);
            }

            } */
        }
        "convert" => convert(&args.layout, &args.corpus),
        "sfb" => output::print_ngrams(&ngram_vec, stats.bigrams, "SFB".to_string(), &args),
        "sfr" => output::print_ngrams(&ngram_vec, stats.bigrams, "SFR".to_string(), &args),
        "sfs" => output::print_ngrams(&ngram_vec, stats.skipgrams, "SFS".to_string(), &args),
        "lsbs" => output::print_ngrams(&ngram_vec, stats.bigrams, "LSB".to_string(), &args),
        "lss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "LSS".to_string(), &args),
        "fsb" => output::print_ngrams(&ngram_vec, stats.bigrams, "FSB".to_string(), &args),
        "hsb" => output::print_ngrams(&ngram_vec, stats.bigrams, "HSB".to_string(), &args),
        "fss" => output::print_ngrams(&ngram_vec, stats.skipgrams, "FSS".to_string(), &args),
        "alt" => output::print_ngrams(&ngram_vec, stats.trigrams, "Alt".to_string(), &args),
        "inroll" => output::print_ngrams(&ngram_vec, stats.trigrams, "Inroll".to_string(), &args),
        "outroll" => output::print_ngrams(&ngram_vec, stats.trigrams, "Outroll".to_string(), &args),
        "inthreeroll" => {
            output::print_ngrams(&ngram_vec, stats.trigrams, "Inthreeroll".to_string(), &args);
        }
        "outthreeroll" => {
            output::print_ngrams(
                &ngram_vec,
                stats.trigrams,
                "Outthreeroll".to_string(),
                &args,
            );
        }
        "red" => output::print_ngrams(&ngram_vec, stats.trigrams, "Red".to_string(), &args),
        "weak" => output::print_ngrams(&ngram_vec, stats.trigrams, "Weak".to_string(), &args),
        "thumb" => output::print_ngrams(&ngram_vec, stats.trigrams, "Thumb".to_string(), &args),
        "bigrams" => output::print_ngrams(&ngram_vec, stats.bigrams, "Bigrams".to_string(), &args),
        "skipgrams" => {
            output::print_ngrams(&ngram_vec, stats.skipgrams, "Skipgrams".to_string(), &args)
        }
        "trigrams" => {
            output::print_ngrams(&ngram_vec, stats.trigrams, "Trigrams".to_string(), &args)
        }
        _ => println!("invalid command"),
    }
}

fn filter_corpus(corpus_name: &String, layout_raw: &[char; 32]) -> String {
    println!("{}", "corpora/raw/".to_owned() + corpus_name);
    let corpus: String = fs::read_to_string("corpora/raw/".to_owned() + corpus_name)
        .expect("error reading corpus")
        .replace("\n\n", "")
        .replace(' ', "_")
        .chars()
        .flat_map(|ch| {
            if ch.is_ascii_uppercase() {
                // Replace uppercase letters with "*" followed by lowercase
                format!("*{}", ch.to_ascii_lowercase())
                    .chars()
                    .collect::<Vec<_>>()
            } else {
                vec![ch]
            }
        })
        .filter(|ch| layout_raw.contains(ch))
        .collect();
    let mut write_file =
        File::create("corpora/filtered/".to_owned() + corpus_name).expect("couldn't write corpus");
    let _ = write_file.write_all(&corpus.as_bytes());
    corpus.to_string()
}

fn load_corpus(corpus_name: &String, layout_name: &String) -> String {
    let layout = load_layout(layout_name);
    match fs::read_to_string("corpora/filtered/".to_owned() + corpus_name) {
        Ok(corpus) => corpus,
        Err(_) => {
            println!("couldn't find corpus, now loading");
            filter_corpus(corpus_name, &layout)
        }
    }
}

fn convert(new_layout_name: &String, corpus_name: &String) {
    let old_layout_name: &String = &String::from("whirl.txt");
    let old_layout = load_layout(&old_layout_name);
    let new_layout = load_layout(&new_layout_name);
    let old_magic_rules = load_magic_rules(&old_layout_name);
    let new_magic_rules = load_magic_rules(&new_layout_name);
    let mut corpus = load_corpus(corpus_name, old_layout_name);

    for letter in new_layout {
        let rule: [char; 2] = match new_magic_rules.get(&letter) {
            Some(other_letter) => [letter, *other_letter],
            None => [letter, letter],
        };
        corpus = corpus.replace(&rule.iter().collect::<String>(), &format!("{letter}*"));
    }

    let hash = new_layout.iter().zip(old_layout).collect::<HashMap<_, _>>();

    let mut new_corpus: String = corpus
        .chars()
        .map(|c| hash.get(&c).copied().unwrap_or(c))
        .collect();

    for letter in old_layout {
        let rule: [char; 2] = match old_magic_rules.get(&letter) {
            Some(other_letter) => [letter, *other_letter],
            None => [letter, letter],
        };
        new_corpus = new_corpus.replace(&format!("{letter}*"), &rule.iter().collect::<String>());
    }
    new_corpus = new_corpus.replace("_", " ");

    println!("{new_corpus}");
}
