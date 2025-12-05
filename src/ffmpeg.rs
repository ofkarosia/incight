use std::{path::PathBuf, process::Command, time::Instant};

use color_eyre::eyre::{ContextCompat, Result};

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
            println!("Processed: {}", path.display());
        } else {
            eprintln!("Failed to process: {}", path.display());
        }
    }

    Ok(time.elapsed().as_secs_f64())
}
