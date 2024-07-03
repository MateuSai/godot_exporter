use std::{
    fs::File,
    path::{Path, PathBuf},
};

use crate::{cli::Cli, exporters::TMP_DIR_NAME};

use super::{ExportPreset, Exporter};

#[derive(Debug)]
pub struct Conf {
    pub project_name: String,
    pub project_version: String,
    pub project_icon: PathBuf,
}

pub struct LinuxExporter {
    pub conf: Conf,
    pub preset: ExportPreset,
}

impl Exporter for LinuxExporter {
    fn package(&self, cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
        println!("conf: {:?}", self.conf);

        let executable_path = PathBuf::from(&cli.output_folder)
            .join(TMP_DIR_NAME)
            .join(Path::new(
                self.conf
                    .project_name
                    .to_lowercase()
                    .replace(" ", "_")
                    .replace("/", "_")
                    .as_str(),
            ));

        Self::export(
            cli,
            &self.preset,
            self.conf
                .project_name
                .to_lowercase()
                .replace(" ", "_")
                .replace("/", "_"),
        )?;

        if !self.preset.steam {
            println!("Creating .desktop file...");

            std::fs::write(
                executable_path.with_extension("desktop"),
                get_desktop_file_text(&self.conf.project_name),
            )?;

            println!(".desktop file created!");

            println!("Creating install script...");

            std::fs::write(
                PathBuf::from(&cli.output_folder).join("install.sh"),
                get_install_script_text(&self.conf.project_name),
            )?;

            println!("install script created!");

            println!("Coppying icon");
            std::fs::copy(
                &self.conf.project_icon,
                PathBuf::from(&cli.output_folder)
                    .join(TMP_DIR_NAME)
                    .join(&self.conf.project_icon.file_name().unwrap()),
            )?;
        }

        let files = Self::get_exported_files(&cli.output_folder)?;
        println!("Exported files: {:?}", files);

        let container_path = PathBuf::from(&cli.output_folder).join(format!(
            "{}_{}_{}",
            executable_path
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
            println!("Creating tar.gz...");

            let tar_path = container_path.with_extension("tar.gz");
            println!("tar path: {}", &tar_path.to_str().unwrap());
            let tar_gz = File::create(tar_path)?;
            let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
            let mut tar = tar::Builder::new(enc);
            for file_path in files {
                println!("Adding {} to tar.gz...", file_path.to_str().unwrap());
                tar.append_file(file_path.file_name().unwrap(), &mut File::open(&file_path)?)?;
                std::fs::remove_file(&file_path)?;
                println!("Added {} to tar.gz", file_path.to_str().unwrap());
            }

            /*  tar.append_file(
                executable_path.with_extension("png").file_name().unwrap(),
                &mut File::open(&self.conf.project_icon)?,
            )?; */

            println!("tar.gz created!");
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

fn get_desktop_file_text(project_name: &str) -> String {
    format!(
        "[Desktop Entry]
Type=Application
Name={}
Exec=sh -c /usr/local/bin/{}
Categories=Game;",
        project_name,
        project_name.to_lowercase().replace(" ", "_")
    )
}

fn get_install_script_text(project_name: &str) -> String {
    let project_name_without_spaces_and_lowercase = project_name.to_lowercase().replace(" ", "_");
    format!(
        "#!/bin/sh

BINARY_DIRECTORY=/usr/local/bin/

cp -f {} $BINARY_DIRECTORY/{}
cp -f {}.pck $BINARY_DIRECTORY/{}.pck
cp -f {}.png /usr/share/pixmaps/{}.png
cp -f {}.desktop /usr/share/applications/{}.desktop
chmod u+x /usr/share/applications/{}.desktop",
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
        project_name_without_spaces_and_lowercase,
    )
}
