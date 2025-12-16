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
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let expected_path = self.output_dir.join(&output_file);
        YoutubeDl::new(url)
            .output_directory(self.output_dir.to_string_lossy().into_owned())
            .output_template(output_file.clone())
            .socket_timeout("15")
            .run()?;

        Ok(expected_path)
    }
}
