use crate::Stats;
use tabled::{settings::Style, builder::Builder};

pub fn print_ngrams(vec: Vec<([char; 3], u32)>, ngrams: u32, title: String) {
    let min_range = 0;
    let max_range = 10;
    let mut builder = Builder::default();
    builder.push_record([title, "Frequency".to_string()]);
    for line in vec.iter().take(max_range).skip(min_range) {
        builder.push_record([
            line.0.iter().collect(),
            (line.1 as f32 / ngrams as f32 * 100.0).to_string(),
        ]);
    }
    let mut table = builder.build();
    table.with(Style::sharp());
    println!("{}", table);
}

pub fn print_stats(stats: Stats) {
    let sfbpercent = stats.sfb as f32 * 100.0 / stats.bigrams as f32;
    let sfspercent = stats.sfs as f32 * 100.0 / stats.skipgrams as f32;
    let lsbpercent = stats.lsb as f32 * 100.0 / stats.bigrams as f32;
    let lsspercent = stats.lss as f32 * 100.0 / stats.skipgrams as f32;
    let fsbpercent = stats.fsb as f32 * 100.0 / stats.bigrams as f32;
    let fsspercent = stats.fss as f32 * 100.0 / stats.skipgrams as f32;
    let altpercent = stats.alt as f32 * 100.0 / stats.trigrams as f32;
    let inrollpercent = stats.inroll as f32 * 100.0 / stats.trigrams as f32;
    let outrollpercent = stats.outroll as f32 * 100.0 / stats.trigrams as f32;
    let inthreerollpercent = stats.inthreeroll as f32 * 100.0 / stats.trigrams as f32;
    let outthreerollpercent = stats.outthreeroll as f32 * 100.0 / stats.trigrams as f32;
    let weakredpercent = stats.weak_red as f32 * 100.0 / stats.trigrams as f32;
    let redpercent = stats.red as f32 * 100.0 / stats.trigrams as f32;
    println!(
        "{}",
        format_args!(
            concat!(
                "SFB: {}%\n",
                "SFS: {}%\n",
                "LSB: {}%\n",
                "LSS: {}%\n",
                "FSB: {}%\n",
                "FSS: {}%\n",
                "Alt: {}%\n",
                "Roll: {}%\n",
                "  In: {}%\n",
                "  Out: {}%\n",
                "Threeroll: {}%\n",
                "  In: {}%\n",
                "  Out: {}%\n",
                "Redirects: {}%\n",
                "Weak Redirects: {}%\n"
            ),
            sfbpercent,
            sfspercent,
            lsbpercent,
            lsspercent,
            fsbpercent,
            fsspercent,
            altpercent,
            inrollpercent + outrollpercent,
            inrollpercent,
            outrollpercent,
            inthreerollpercent + outthreerollpercent,
            inthreerollpercent,
            outthreerollpercent,
            redpercent + weakredpercent,
            weakredpercent
        )
    )
}
