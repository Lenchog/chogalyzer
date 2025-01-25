use crate::Stats;
use tabled::{col, builder::Builder, settings::Style};

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

pub fn print_stats(stats: Stats, layout: [char; 32], magic_rules: &Vec<String>, layout_name: String) {
    let layout_rows: [String; 3] = [layout[0..10].iter().collect(), layout[10..20].iter().collect(), layout[20..30].iter().collect()];
    let mut layout_builder = Builder::default();

    layout_builder.push_record(["Layout"]);
    for row in layout_rows { 
        let mut row_format: String = Default::default();
        for char in row.chars() {
            row_format.push(char);
            row_format.push(' ');
        }
        row_format.insert(10, ' ');
        layout_builder.push_record([&row_format]);
    }
    layout_builder.push_record([("      ".to_owned() + &layout[30].to_string() + "      " + &layout[31].to_string())]);

    for rule in magic_rules {
        layout_builder.push_record([rule]);
    }
    let mut layout_table = layout_builder.build();

    layout_table.with(Style::sharp());

    let sfbpercent = stats.sfb as f32 * 100.0 / stats.bigrams as f32;
    let sfrpercent = stats.sfr as f32 * 100.0 / stats.bigrams as f32;
    let sfspercent = stats.sfs as f32 * 100.0 / stats.skipgrams as f32;
    let lsbpercent = stats.lsb as f32 * 100.0 / stats.bigrams as f32;
    let lsspercent = stats.lss as f32 * 100.0 / stats.skipgrams as f32;
    let fsbpercent = stats.fsb as f32 * 100.0 / stats.bigrams as f32;
    let fsspercent = stats.fss as f32 * 100.0 / stats.skipgrams as f32;
    let altpercent = stats.alt as f32 * 100.0 / stats.trigrams as f32;
    let inrollpercent = stats.inroll as f32 * 100.0 / stats.trigrams as f32;
    let outrollpercent = stats.outroll as f32 * 100.0 / stats.trigrams as f32;
    let rollpercent = (stats.inroll + stats.outroll) as f32 * 100.0 / stats.trigrams as f32;
    let inthreerollpercent = stats.inthreeroll as f32 * 100.0 / stats.trigrams as f32;
    let outthreerollpercent = stats.outthreeroll as f32 * 100.0 / stats.trigrams as f32;
    let threerollpercent = (stats.inthreeroll + stats.outthreeroll) as f32 * 100.0 / stats.trigrams as f32;
    let inrolltalpercent = (stats.inroll + stats.inthreeroll) as f32 * 100.0 / stats.trigrams as f32;
    let outrolltalpercent = (stats.outroll + stats.outthreeroll) as f32 * 100.0 / stats.trigrams as f32;
    let rolltalpercent = (stats.inroll + stats.outroll + stats.inthreeroll + stats.outthreeroll) as f32 * 100.0 / stats.trigrams as f32;
    let weakredpercent = stats.weak_red as f32 * 100.0 / stats.trigrams as f32;
    let redpercent = (stats.red + stats.weak_red) as f32 * 100.0 / stats.trigrams as f32;

    let mut general = Builder::default();
    general.push_record(["Score", &stats.score.to_string()]);
    general.push_record(["Fspeed", &stats.fspeed.to_string()]);
    general.push_record(["Heatmap", &stats.heatmap.to_string()]);
    general.push_record(["Alt", &(altpercent.to_string() + "%")]);
    let mut general_table = general.build();
    general_table.with(Style::sharp());

    let mut bigram = Builder::default();
    bigram.push_record(["", "Bigram", "Skipgram"]);
    bigram.push_record(["SF", &sfbpercent.to_string(), &sfspercent.to_string()]);
    bigram.push_record(["LS", &lsbpercent.to_string(), &lsspercent.to_string()]);
    bigram.push_record(["FS", &fsbpercent.to_string(), &fsspercent.to_string()]);
    let mut bigram_table = bigram.build();
    bigram_table.with(Style::sharp());

    let mut roll = Builder::default();
    roll.push_record(["", "Inwards", "Outwards", "Total"]);
    roll.push_record(["2", &inrollpercent.to_string(), &outrollpercent.to_string(), &rollpercent.to_string()]);
    roll.push_record(["3", &inthreerollpercent.to_string(), &outthreerollpercent.to_string(), &threerollpercent.to_string()]);
    roll.push_record(["Total", &inrolltalpercent.to_string(), &outrolltalpercent.to_string(), &rolltalpercent.to_string()]);
    let mut roll_table = roll.build();
    roll_table.with(Style::sharp());

    let mut table = col![
        layout_name,
        layout_table,
        "General",
        general_table,
        "Bigram and Skipgram",
        bigram_table,
        "Rolls",
        roll_table
    ];
    table.with(Style::sharp());
    println!("{}", table);
}
