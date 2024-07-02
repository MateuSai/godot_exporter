use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use super::{ExportPreset, Exporter, TMP_DIR_NAME};

pub struct Conf {
    pub project_name: String,
    pub project_version: String,
}

pub struct WindowsExporter {
    pub conf: Conf,
    pub preset: ExportPreset,
}

impl Exporter for WindowsExporter {
    fn package(&self, cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
        let mut project_name = self.conf.project_name.to_owned();
        project_name.push_str("_");
        project_name.push_str(&self.preset.name);
        project_name.push_str("_");
        project_name.push_str(&self.conf.project_version.replace(".", "_"));
        let executable_path = PathBuf::from(&cli.output_folder)
            .join(TMP_DIR_NAME)
            .join(Path::new(
                self.conf
                    .project_name
                    .to_lowercase()
                    .replace(" ", "_")
                    .replace("/", "_")
                    .as_str(),
            ))
            .with_extension("exe");

        Self::export(
            cli,
            &self.preset,
            PathBuf::from(
                self.conf
                    .project_name
                    .to_lowercase()
                    .replace(" ", "_")
                    .replace("/", "_"),
            )
            .with_extension("exe")
            .to_str()
            .unwrap()
            .to_owned(),
        )?;

        let files = Self::get_exported_files(&cli.output_folder)?;
        println!("Exported files: {:?}", files);

        let container_path = PathBuf::from(&cli.output_folder).join(format!(
            "{}_{}_{}",
            executable_path
                .with_extension("")
                .file_name()
                .expect("Linux executable does not have file_name")
                .to_str()
                .unwrap(),
            self.preset
                .name
                .to_lowercase()
                .replace(" ", "_")
                .replace("/", "_"),
            self.conf.project_version.replace(".", "_")
        ));

        if cli.compress {
            println!("Creating zip...");

            let options = FileOptions::default().compression_method(CompressionMethod::DEFLATE);
            let mut zip = ZipWriter::new(File::create(container_path.with_extension("zip"))?);

            for file_path in &files {
                let file = File::open(file_path)?;
                zip.start_file(file_path.file_name().unwrap().to_str().unwrap(), options)?;

                let mut buffer = Vec::new();
                std::io::copy(&mut file.take(u64::MAX), &mut buffer)?;

                zip.write_all(&buffer)?;

                std::fs::remove_file(file_path)?;
            }

            zip.finish()?;

            println!("Done compressing files!");
        } else {
            println!("Adding files inside foder");

            let folder_path = container_path;
            if folder_path.exists() {
                println!("Folder already exists, reusing it...");
            } else {
                std::fs::create_dir(folder_path.to_owned())?;
                println!("Folder created");
            }

            for file_path in &files {
                std::fs::rename(
                    file_path,
                    folder_path.to_owned().join(file_path.file_name().unwrap()),
                )?;
            }
        }

        Ok(())
    }
}
