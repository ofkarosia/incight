use std::{path::PathBuf, process::Command, time::Instant};

use color_eyre::eyre::{ContextCompat, Result};
use colour::{cyan, e_red};

pub fn compress_video(video_paths: &[PathBuf], targets: &[usize], args: &[&str]) -> Result<f64> {
    let time = Instant::now();

    for &i in targets {
        let path = &video_paths[i];
        let stem = path
            .file_stem()
            .context("Failed to get file stem")?
            .to_str()
            .context("Failed to convert OsStr to str")?;
        let ext = path.extension().unwrap().to_str().unwrap();

        let mut cmd = Command::new("ffmpeg");

        cmd.args([
            "-i",
            path.to_str().context("Failed to convert path to str")?,
            "-c:a",
            "copy",
            "-c:v",
            "libx265",
        ]);
        cmd.args(args);

        let ecode = cmd
            .args([
                "-movflags",
                "+faststart",
                "-y",
                &format!("{}_batch.{}", stem, ext),
            ])
            .spawn()?
            .wait()?;

        if ecode.success() {
            cyan!("Processed: ");
            println!("{}", path.display());
            println!("")
        } else {
            e_red!("Failed to process: ");
            eprintln!("{}", path.display());
            eprintln!("")
        }
    }

    Ok(time.elapsed().as_secs_f64())
}
