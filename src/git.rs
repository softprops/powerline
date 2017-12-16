use std::io;

use git2::{self, BranchType, Repository, StatusOptions, StatusShow};
use term::color;

use Segment;

pub fn segments() -> io::Result<Vec<Segment>> {
    let repo = Repository::discover(".").ok();
    let mut segs = vec![];

    // (branch, maybe_local, maybe_upstream)
    let branch_info = repo.as_ref().and_then(|r| {
        r.branches(Some(BranchType::Local))
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(move |(ref branch, _)| {
                if branch.is_head() {
                    let local = branch.get().target();
                    let upstream =
                        branch.upstream().ok().and_then(|b| b.get().target());
                    if let Ok(name) = branch.name() {
                        if let Some(name) = name {
                            return Some((name.to_string(), local, upstream));
                        }
                    }
                }
                None
            })
            .next()
    });

    let statuses = repo.as_ref().and_then(|repo| {
        repo.statuses(Some(
            StatusOptions::new()
                .show(StatusShow::IndexAndWorkdir)
                .include_untracked(true),
        )).ok()
    });
    //let clean = statuses.as_ref().map(|s| s.is_empty()).unwrap_or(false);

    segs.push(Segment::new(
        format!(
            " \u{e0a0} {} ",
            branch_info
                .as_ref()
                .map(|&(ref branch, _, _)| branch)
                .unwrap_or(&"Big Bang".to_string())
        ),
        color::BRIGHT_WHITE,
        color::BRIGHT_BLACK,
    ));

    // # to push / # to pull
    for repo in repo.as_ref() {
        if let Some((_, Some(local), Some(upstream))) = branch_info {
            if let Ok((ahead, behind)) =
                repo.graph_ahead_behind(local, upstream)
            {
                if ahead > 0 {
                    segs.push(Segment::new(
                        format!(
                            " {}\u{2714} ",
                            if ahead > 1 {
                                ahead.to_string()
                            } else {
                                "".into()
                            }
                        ),
                        color::BRIGHT_WHITE,
                        color::BRIGHT_BLACK,
                    ))
                }
                if behind > 0 {
                    segs.push(Segment::new(
                        format!(
                            " {}\u{2714} ",
                            if behind > 1 {
                                behind.to_string()
                            } else {
                                "".into()
                            }
                        ),
                        color::BRIGHT_WHITE,
                        color::BRIGHT_BLACK,
                    ))
                }
            }
        }
    }

    // file stats
    if let Some(statuses) = statuses {
        let mut staged = 0;
        let mut unstaged = 0;
        let mut conflicted = 0;
        let mut untracked = 0;
        for status in statuses.iter() {
            let status = status.status();
            if status.contains(git2::STATUS_INDEX_NEW) ||
                status.contains(git2::STATUS_INDEX_MODIFIED) ||
                status.contains(git2::STATUS_INDEX_TYPECHANGE) ||
                status.contains(git2::STATUS_INDEX_RENAMED) ||
                status.contains(git2::STATUS_INDEX_DELETED)
            {
                staged += 1;
            }
            if status.contains(git2::STATUS_WT_MODIFIED) ||
                status.contains(git2::STATUS_WT_TYPECHANGE) ||
                status.contains(git2::STATUS_WT_DELETED)
            {
                unstaged += 1;
            }
            if status.contains(git2::STATUS_WT_NEW) {
                untracked += 1;
            }
            if status.contains(git2::STATUS_CONFLICTED) {
                conflicted += 1;
            }
        }
        if staged > 0 {
            segs.push(Segment::new(
                format!(
                    " {}\u{2714} ",
                    if staged > 1 {
                        staged.to_string()
                    } else {
                        "".into()
                    }
                ),
                color::BRIGHT_WHITE,
                color::BRIGHT_YELLOW,
            ))
        }
        if unstaged > 0 {
            segs.push(Segment::new(
                format!(
                    " {}\u{270E} ",
                    if unstaged > 1 {
                        unstaged.to_string()
                    } else {
                        "".into()
                    }
                ),
                color::BRIGHT_WHITE,
                color::BRIGHT_RED,
            ))
        }
        if untracked > 0 {
            segs.push(Segment::new(
                format!(
                    " {}? ",
                    if untracked > 1 {
                        untracked.to_string()
                    } else {
                        "".into()
                    }
                ),
                color::BRIGHT_WHITE,
                color::BRIGHT_GREEN,
            ))
        }
    }


    Ok(segs)
}
