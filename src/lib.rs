extern crate term;
extern crate regex;
extern crate git2;

use std::io;
use std::str::FromStr;

use term::color::Color;

mod cwd;
mod git;
mod k8s;

pub fn git() -> io::Result<Vec<Segment>> {
    git::segments()
}

pub fn cwd() -> io::Result<Vec<Segment>> {
    cwd::segments()
}

pub fn k8s() -> Segment {
    k8s::segments()
}

pub enum SegmentType {
    Cwd,
    Git,
    K8s,
}

impl FromStr for SegmentType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cwd" => Ok(SegmentType::Cwd),
            "git" => Ok(SegmentType::Git),
            "k8s" => Ok(SegmentType::K8s),
            _ => Err(()),
        }
    }
}

/// a unit of display within a powerline
#[derive(Debug, Default)]
pub struct Segment {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    pub separator: Option<String>,
    pub separator_fg: Option<Color>,
}

impl Segment {
    pub fn new<C>(content: C, fg: Color, bg: Color) -> Self
    where
        C: Into<String>,
    {
        Self {
            content: content.into(),
            fg,
            bg,
            ..Default::default()
        }
    }
}
