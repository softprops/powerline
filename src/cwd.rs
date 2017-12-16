use term;

use super::Segment;

use std::{env, io};
use std::path::{Path, PathBuf};
use term::color;

const HOME: &str = "~";

fn normalize_cwd() -> io::Result<PathBuf> {
    let cd = env::current_dir()?;
    if let Some(home) = env::home_dir() {
        if cd.starts_with(&home) {
            let stripped = cd.strip_prefix(&home).unwrap();
            let mut normal = Path::new(HOME).to_path_buf();
            normal.push(&stripped);
            return Ok(normal);
        }
    }
    Ok(cd)
}

pub fn segments() -> io::Result<Vec<Segment>> {
    let norm = normalize_cwd();
    let path = norm.iter().map(|s| s.to_string_lossy()).collect::<Vec<_>>();
    Ok(path.iter().enumerate().fold(vec![], |mut segs, (i, p)| {
        let last = i == path.len() - 1;
        segs.push(Segment {
            content: format!(" {} ", p),
            fg: color::BRIGHT_WHITE,
            bg: if HOME == p {
                color::BRIGHT_MAGENTA
            } else {
                color::CYAN
            },
            separator: if !last && "~" != p {
                Some("\u{E0B1}".into())
            } else {
                None
            },
            separator_fg: if !last && p != HOME {
                Some(term::color::BRIGHT_WHITE)
            } else {
                None
            },
        });
        segs
    }))
}
