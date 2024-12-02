use cargo_metadata::MetadataCommand;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(name = "Advent of code")]
pub struct Arg {
    /// which year to run problems from
    #[arg(short, long, value_parser = valid_year)]
    year: u16,

    /// which day of the year to run problems from
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// which part of the problem to run, default to all parts
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,
}

fn valid_year(s: &str) -> Result<u16, String> {
    let metadata = MetadataCommand::new()
        .exec()
        .map_err(|_| "failed to get workspace metadata")?;

    let files = std::fs::read_dir(&metadata.workspace_root).map_err(|_| "failed to read workspace root directory")?;

    let mut available_range = (u16::MAX, 0);
    for file in files.flatten() {
        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();
        if file.metadata().map_err(|_| "failed to get file metadata")?.is_dir() && file_name.starts_with("aoc-") {
            let (_, year) = file_name.split_once("-").unwrap();
            let Ok(year) = year.parse::<u16>() else {
                continue;
            };
            available_range.0 = available_range.0.min(year);
            available_range.1 = available_range.1.max(year);
        }
    }

    let year_specified = s
        .parse::<u16>()
        .map_err(|_| format!("{s} is not a valid year number"))?;

    if (available_range.0..=available_range.1).contains(&year_specified) {
        Ok(year_specified)
    } else {
        Err(format!(
            "the year `{year_specified}` is not in range {}..={}",
            available_range.0, available_range.1
        ))
    }
}

fn main() -> Result<(), String> {
    let args = Arg::parse();

    match &args.year {
        2023 => aoc_2023::run_day(args.day, args.part)?,
        2024 => aoc_2024::run_day(args.day, args.part)?,
        _ => {}
    };

    Ok(())
}
