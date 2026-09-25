mod parser;
mod plot;

use std::{path::PathBuf, process::ExitCode, str::FromStr};

use anyhow::{Ok, Result};
use parser::parse_csv;
use plot::plot_histogram;

const FIGURE_DIR: &str = "figure";

fn main() -> Result<ExitCode> {
    let year = format!("year{}", std::env::var("AOC_YEAR")?);
    let dir = PathBuf::from_str(year.as_str())?;
    if !dir.exists() {
        eprintln!("{} doesn't exist", dir.display());
        return Ok(ExitCode::FAILURE);
    }

    let subdirs = dir.read_dir()?;
    let mut days = vec![];
    for subdir in subdirs {
        let subdir = subdir?;
        let dir_name = subdir.file_name();
        if !dir_name.to_string_lossy().starts_with("day") {
            continue;
        }
        let day = dir_name.to_string_lossy()[3..].parse::<usize>()?;
        let csv_path = subdir.path().join("benchmark.csv");
        if !csv_path.exists() {
            eprintln!("{subdir:?} exists but benchmark results were not found!");
            continue;
        }
        let csv = std::fs::read_to_string(csv_path)?;
        let (_, results) =
            parse_csv(&csv).map_err(|err| anyhow::anyhow!("Failed to parse csv: {err:?}"))?;
        if results.len() < 3 {
            eprintln!("Benchmark results are not complete");
        }
        let (parse, part1, part2) = (results[0], results[1], results[2]);
        days.push((day, (parse, part1, part2)));
    }

    let fig_dir = PathBuf::from_str(FIGURE_DIR)?;
    if !fig_dir.exists() {
        std::fs::create_dir(&fig_dir)?;
    }

    let figure_path = fig_dir.join(format!("{year}.svg"));
    plot_histogram(figure_path, &days, false)?;

    Ok(ExitCode::SUCCESS)
}
