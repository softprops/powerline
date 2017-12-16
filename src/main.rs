extern crate term;
extern crate powerline;
extern crate clap;

use std::io::{self, Write};

use clap::{App, Arg};
use powerline::SegmentType;

pub fn run(types: Vec<SegmentType>) -> io::Result<()> {
    let segments = types.into_iter().fold(Vec::new(), |mut acc, s| {
        match s {
            SegmentType::Cwd => acc.extend(powerline::cwd().unwrap()),
            SegmentType::Git => acc.extend(powerline::git().unwrap()),
            SegmentType::K8s => acc.push(powerline::k8s()),
        }
        acc
    });
    let mut t = term::stdout().unwrap();
    for (i, s) in segments.iter().enumerate() {
        t.fg(s.fg)?;
        t.bg(s.bg)?;
        write!(t, "{}", s.content)?;
        if let Some(next) = segments.get(i + 1) {
            t.bg(next.bg)?;
        } else {
            t.reset()?;
        }
        t.fg(s.separator_fg.unwrap_or(s.bg))?;
        write!(t, "{}", s.separator.clone().unwrap_or("\u{E0B0}".into()))?;
        t.reset()?;
    }
    writeln!(t, " ")?;
    Ok(())
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about("extends your cmd line with super powers")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("segments")
                .long("segments")
                .help("The list of segments to load, separated by ','")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["cwd", "git", "k8s"])
                .value_delimiter(",")
                .default_value("cwd,git,k8s"),
        )
        .get_matches();
    let segments: Vec<SegmentType> = matches
        .values_of("segments")
        .unwrap()
        .map(|segment| segment.parse().unwrap())
        .collect();
    run(segments).unwrap();
}