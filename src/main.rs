use std::{env, path::PathBuf};

use color_eyre::eyre::Result;
use demand::{DemandOption, MultiSelect, Select};
use phf::{Set, phf_set};
use strum::VariantArray;

use crate::{ffmpeg::compress_video, preset::Preset};

mod ffmpeg;
mod preset;

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

fn get_custom_preset_args() -> Result<(String, String, String)> {
    let preset = Select::new("Select a built-in preset")
        .options(vec![
            DemandOption::new("fast"),
            DemandOption::new("medium"),
            DemandOption::new("slow"),
        ])
        .run()?
        .to_string();

    let crf = Select::new("Select CRF value")
        .options(vec![
            DemandOption::new("26"),
            DemandOption::new("27"),
            DemandOption::new("28"),
        ])
        .run()?
        .to_string();

    let params = MultiSelect::new("Select additional x265 parameters")
        .options(vec![
            DemandOption::new("aq-mode=3"),
            DemandOption::new("rd=4"),
        ])
        .run()?
        .join(":");

    Ok((preset, crf, params))
}

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
        .filterable(true)
        .options(options)
        .run()?;

    if list.is_empty() {
        println!("No video selected.");
        return Ok(());
    }

    let preset_options = Preset::VARIANTS
        .iter()
        .map(DemandOption::new)
        .collect::<Vec<_>>();
    let preset = Select::new("Select a preset")
        .options(preset_options)
        .run()?;
    let preset_args = preset.get_args();

    let elapsed = if let Some(args) = preset_args {
        compress_video(&video_paths, &list, args)?
    } else {
        let (preset, crf, x265_params) = get_custom_preset_args()?;
        compress_video(
            &video_paths,
            &list,
            &[
                "-preset",
                &preset,
                "-crf",
                &crf,
                "-x265-params",
                &x265_params,
            ],
        )?
    };

    println!(
        "All compression task completed. Time elapsed: {:.2}s",
        elapsed
    );

    Ok(())
}
