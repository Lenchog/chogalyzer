use crate::Args;
use crate::Stats;
use ahash::AHashMap;
use tabled::{builder::Builder, col, settings::Style};

/// When the user wants a list of the most frequent of a type of ngram, displays them in a table
pub fn print_ngrams(vec: &[([char; 3], u32)], ngrams: u32, title: String, args: &Args) {
    #![allow(clippy::cast_precision_loss)]
    let min_range = 0;
    let max_range = 10;
    if !args.compact {
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
        println!("{table}");
    } else {
        let mut output = String::new();
        for line in vec.iter().take(max_range).skip(min_range) {
            output.push_str(
                format!(
                    "{}{}\n",
                    line.0.iter().collect::<String>(),
                    (line.1 as f32 / ngrams as f32 * 100.0)
                )
                .as_str(),
            );
        }
        println!("{output}");
    }
}

/// What's needed to display a layout
pub struct LayoutDisplay<'a> {
    name: &'a str,
    layout: [String; 4],
    stats: AHashMap<&'a str, f32>,
    magic_rules: Vec<String>,
}
impl LayoutDisplay<'_> {
    /// Get the data needed for display
    pub fn new<'a>(
        name: &'a str,
        layout: [char; 32],
        stats: &'a Stats,
        magic_rules: &'a AHashMap<char, char>,
    ) -> LayoutDisplay<'a> {
        LayoutDisplay {
            name,
            layout: format_layout(layout),
            stats: get_stats_hash(stats),
            magic_rules: format_magic(magic_rules),
        }
    }
    /// Displays data in a more simple way.
    // TODO
    pub fn simple_display(_display: Self) {
        todo!()
    }

    /// Display a lot of information about a layout in a pretty table
    pub fn full(self) {
        let mut layout_builder = Builder::default();
        layout_builder.push_record(["Layout"]);
        for row in self.layout {
            let mut row_format: String = String::default();
            for char in row.chars() {
                row_format.push(char);
                row_format.push(' ');
            }
            row_format.insert(10, ' ');
            layout_builder.push_record([&row_format]);
        }

        for rule in self.magic_rules {
            layout_builder.push_record([rule]);
        }
        let mut layout_table = layout_builder.build();

        layout_table.with(Style::sharp());
        let general_stats = vec![
            "Score", "Fspeed", "Heatmap", "Alt", "SFR", "Red", "Weak Red",
        ];
        let general = table_from_hashmap(general_stats, self.stats.clone());

        let mut bigram = Builder::default();
        bigram.push_record(["", "Bigram", "Skipgram"]);
        let stats = ["SF", "LS", "FS", "HS"];
        for stat in stats {
            let (name_b, name_s) = (stat.to_owned() + "B", stat.to_owned() + "S");
            bigram.push_record([
                stat,
                self.stats
                    .get(name_b.as_str())
                    .expect("")
                    .to_string()
                    .as_str(),
                self.stats
                    .get(&name_s.as_str())
                    .expect("")
                    .to_string()
                    .as_str(),
            ]);
        }
        let mut bigram_table = bigram.build();
        bigram_table.with(Style::sharp());

        let mut roll = Builder::default();
        roll.push_record(["", "Inwards", "Outwards", "Total"]);
        roll.push_record([
            "2",
            &self.stats.get("Inroll").expect("").to_string(),
            &self.stats.get("Outroll").expect("").to_string(),
            &self.stats.get("2Roll").expect("").to_string(),
        ]);
        roll.push_record([
            "3",
            &self.stats.get("In3Roll").expect("").to_string(),
            &self.stats.get("Out3Roll").expect("").to_string(),
            &self.stats.get("3Roll").expect("").to_string(),
        ]);
        roll.push_record([
            "Total",
            &self.stats.get("InrollTal").expect("").to_string(),
            &self.stats.get("OutrollTal").expect("").to_string(),
            &self.stats.get("Roll").expect("").to_string(),
        ]);
        let mut roll_table = roll.build();
        roll_table.with(Style::sharp());

        let mut table = col![
            self.name,
            layout_table,
            "General",
            general,
            "Bigram and Skipgram",
            bigram_table,
            "Rolls",
            roll_table
        ];
        table.with(Style::sharp());
        println!("{table}");
    }
}

/// Format a layout for display
fn format_layout(layout: [char; 32]) -> [String; 4] {
    [
        layout[0..10].iter().collect(),
        layout[10..20].iter().collect(),
        layout[20..30].iter().collect(),
        ("   ".to_owned() + &layout[30].to_string() + " " + &layout[31].to_string()),
    ]
}

/// Format magic rules for display
fn format_magic(magic_rules: &AHashMap<char, char>) -> Vec<String> {
    magic_rules
        .iter()
        .map(|(&k, &v)| format!("{}{}", k, v))
        .collect()
}

/// Get a table for display from the ngram HashMap
fn table_from_hashmap(stats: Vec<&str>, hash: AHashMap<&str, f32>) -> tabled::Table {
    let mut builder = Builder::default();

    for stat in stats {
        builder.push_record([stat, &hash.get(stat).expect("").to_string()]);
    }
    let mut table = builder.build();
    table.with(Style::sharp());
    table
}

/// Turn the stats into a hashmap for display. I don't know why this is needed tbh
fn get_stats_hash(stats: &Stats) -> AHashMap<&str, f32> {
    #![allow(clippy::cast_precision_loss)]
    AHashMap::from([
        ("SFB", stats.sfb as f32 * 100.0 / stats.chars as f32),
        ("SFR", stats.sfr as f32 * 100.0 / stats.chars as f32),
        ("SFS", stats.sfs as f32 * 100.0 / stats.skipgrams as f32),
        ("LSB", stats.lsb as f32 * 100.0 / stats.chars as f32),
        ("LSS", stats.lss as f32 * 100.0 / stats.skipgrams as f32),
        ("HSB", stats.hsb as f32 * 100.0 / stats.chars as f32),
        ("HSS", stats.hss as f32 * 100.0 / stats.skipgrams as f32),
        ("FSB", stats.fsb as f32 * 100.0 / stats.chars as f32),
        ("FSS", stats.fss as f32 * 100.0 / stats.skipgrams as f32),
        ("Alt", stats.alt as f32 * 100.0 / stats.chars as f32),
        ("Inroll", stats.inroll as f32 * 100.0 / stats.chars as f32),
        ("Outroll", stats.outroll as f32 * 100.0 / stats.chars as f32),
        (
            "2Roll",
            (stats.inroll + stats.outroll) as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "In3Roll",
            stats.inthreeroll as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "Out3Roll",
            stats.outthreeroll as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "3Roll",
            (stats.inthreeroll + stats.outthreeroll) as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "InrollTal",
            (stats.inroll + stats.inthreeroll) as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "OutrollTal",
            (stats.outroll + stats.outthreeroll) as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "Roll",
            (stats.inroll + stats.outroll + stats.inthreeroll + stats.outthreeroll) as f32 * 100.0
                / stats.chars as f32,
        ),
        (
            "Weak Red",
            stats.weak_red as f32 * 100.0 / stats.chars as f32,
        ),
        (
            "Red",
            (stats.red + stats.weak_red) as f32 * 100.0 / stats.chars as f32,
        ),
        ("Score", stats.score as f32),
        ("Fspeed", stats.fspeed as f32),
        ("Heatmap", stats.heatmap as f32),
        ("Finger usage penalty", stats.column_pen as f32),
    ])
}
