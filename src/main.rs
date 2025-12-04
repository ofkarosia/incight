use std::{env, path::PathBuf, process::Command};

use color_eyre::eyre::{ContextCompat, Result};
use demand::{DemandOption, MultiSelect};
use phf::{Set, phf_set};

static EXT_LIST: Set<&'static str> = phf_set! {
    "mp4",
    "mkv",
    "avi",
    "mov",
    "flv",
    "wmv",
    "webm",
    "ts",
    "ogv",
    "mpg",
    "mpeg"
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let video_paths: Vec<PathBuf> = env::current_dir()?
        .read_dir()?
        .filter_map(|e| {
            let entry = e.ok()?;
            let path = entry.path();
            let ext = path.extension()?.to_str()?;

            if EXT_LIST.contains(ext) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let options: Vec<DemandOption<usize>> = video_paths
        .iter()
        .enumerate()
        .filter_map(|(index, s)| Some(DemandOption::new(s.to_str()?).item(index)))
        .collect();

    let list = MultiSelect::new("Select video to process")
        .options(options)
        .run()?;

    if list.is_empty() {
        println!("No video selected.");
        return Ok(());
    }

    for i in list {
        let path = &video_paths[i];
        let stem = path
            .file_stem()
            .context("Failed to get file stem")?
            .to_str()
            .context("Failed to convert OsStr to str")?;
        let ext = path.extension().unwrap().to_str().unwrap();

        let ecode = Command::new("ffmpeg")
            .args([
                "-i",
                path.to_str().context("Failed to convert path to str")?,
                "-c:a",
                "copy",
                "-c:v",
                "libx265",
                "-crf",
                "23.5",
                "-preset",
                "fast",
                &format!("{}_batch.{}", stem, ext),
            ])
            .spawn()?
            .wait()?;

        if ecode.success() {
            println!("Processed: {}", path.display());
        } else {
            eprintln!("Failed to process: {}", path.display());
        }
    }

    Ok(())
}
