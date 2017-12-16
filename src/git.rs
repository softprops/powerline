use std::{env, io, process};
use std::collections::HashMap;

use regex::Regex;
use term::color;

use Segment;


const BRANCH_INFO: &str =
    r"^## (?P<local>\S+?)(\.{3}(?P<remote>\S+?)( \[(ahead (?P<ahead>\d+)(, )?)?(behind (?P<behind>\d+))?])?)?$";

fn status() -> io::Result<String> {
    let mut cmd = process::Command::new("git");
    cmd.arg("status").arg("--porcelain").arg("-b").env(
        "LANG",
        "C",
    );
    if let Some(home) = env::home_dir() {
        cmd.env("HOME", home);
    }
    if let Some(path) = env::var("PATH").ok() {
        cmd.env("PATH", path);
    }
    Ok(String::from_utf8(cmd.output()?.stdout).unwrap())
}

pub fn segments() -> io::Result<Vec<Segment>> {
    let status = status()?;
    // parse untracked, confliects, staged, and unstaged
    let mut result = status.lines().skip(1).filter(|l| l.len() > 0).fold(
        HashMap::new(),
        |mut result, line| {
            match &line[..2] {
                "??" => {
                    *result.entry("untracked").or_insert(0) += 1;
                }
                "UU" => {
                    *result.entry("conflicts").or_insert(0) += 1;
                }
                "DD" | "AU" | "UD" | "UA" | "DU" | "AA" => (),
                code => {
                    if code.chars().nth(0) != Some(' ') {
                        *result.entry("staged").or_insert(0) += 1;
                    }
                    if code.chars().nth(1) != Some(' ') {
                        *result.entry("unstaged").or_insert(0) += 1;
                    }
                }
            }
            result
        },
    );

    let branch = status.lines().next().and_then(|line| {
        let re = Regex::new(BRANCH_INFO).unwrap();
        re.captures(line).and_then(|caps| {
            caps.name("local").map(|m| {
                let branch = m.as_str();
                if let Some(ahead) = caps.name("ahead") {
                    *result.entry("ahead").or_insert(0) +=
                        ahead.as_str().parse().unwrap()
                }
                if let Some(behind) = caps.name("behind") {
                    *result.entry("behind").or_insert(0) +=
                        behind.as_str().parse().unwrap()
                }
                branch
            })
        })
    });

    let mut segs = vec![];
    segs.push(Segment::new(
        format!(" \u{e0a0} {} ", branch.unwrap_or("Big Bang")),
        color::BRIGHT_WHITE,
        color::BRIGHT_BLACK,
    ));
    if let Some(staged) = result.get("staged") {
        segs.push(Segment::new(
            format!(
                " {}\u{2714} ",
                if *staged > 1 {
                    staged.to_string()
                } else {
                    "".into()
                }
            ),
            color::BRIGHT_WHITE,
            color::BRIGHT_YELLOW,
        ))
    }
    if let Some(unstaged) = result.get("unstaged") {
        segs.push(Segment::new(
            format!(
                " {}\u{270E} ",
                if *unstaged > 1 {
                    unstaged.to_string()
                } else {
                    "".into()
                }
            ),
            color::BRIGHT_WHITE,
            color::BRIGHT_RED,
        ))
    }
    if let Some(untracked) = result.get("untracked") {
        segs.push(Segment::new(
            format!(
                " {}? ",
                if *untracked > 1 {
                    untracked.to_string()
                } else {
                    "".into()
                }
            ),
            color::BRIGHT_WHITE,
            color::BRIGHT_GREEN,
        ))
    }
    Ok(segs)
}
