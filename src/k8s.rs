use Segment;

use std::process;
use term::color;

pub fn segments() -> Segment {
    let cmd = process::Command::new("kubectl")
        .arg("config")
        .arg("current-context")
        .output();
    Segment::new(
        format!(
            " \u{2699} {} ",
            String::from_utf8_lossy(&cmd.unwrap().stdout).replace("\n", "")
        ),
        color::BRIGHT_WHITE,
        color::BRIGHT_YELLOW,
    )
}
