use std::path::PathBuf;
use youtube_dl::YoutubeDl;

#[derive(Clone)]
pub struct Downloader {
    output_dir: PathBuf,
}

impl Downloader {
    pub fn new(output_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&output_dir)
            .expect("failed to create output directory");
        Self { output_dir }
    }

    pub fn download(
        &self,
        url: String,
        output_file: String,
        only_sound: bool,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let before: std::collections::HashSet<PathBuf> = std::fs::read_dir(&self.output_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        let mut cmd = YoutubeDl::new(url);
        cmd.output_template(&output_file)
            .socket_timeout("15");

        if only_sound {
            cmd.extract_audio(true);
        }

        cmd.download_to(self.output_dir.to_str().unwrap())?;
    
        let prev_dir = std::env::current_dir()?;
        std::env::set_current_dir(&self.output_dir)?;
        let run_result = cmd.run();
        std::env::set_current_dir(prev_dir)?;
        run_result?;

        let after: std::collections::HashSet<PathBuf> = std::fs::read_dir(&self.output_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        let actual = after
            .difference(&before)
            .next()
            .cloned()
            .ok_or("Could not find downloaded file")?;

        Ok(actual)
    }
}
