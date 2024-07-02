use std::{
    path::PathBuf,
    process::{Command, Output, Stdio},
};

use ini::Ini;

use crate::cli::Cli;

mod linux;
mod windows;

static TMP_DIR_NAME: &str = "godot_export_tmp_dir";

pub trait Exporter {
    fn export(
        cli: &Cli,
        export_preset: &ExportPreset,
        executable_file_name: String,
    ) -> Result<Output, std::io::Error> {
        let tmp_dir_path = PathBuf::from(&cli.output_folder).join(TMP_DIR_NAME);

        if tmp_dir_path.exists() {
            println!("Tmp directory already exists, removing it...");
            std::fs::remove_dir_all(&tmp_dir_path)?;
        }

        std::fs::create_dir(&tmp_dir_path)?;

        Command::new(&cli.godot_path)
            .args([
                "--headless",
                "--path",
                cli.project_path.as_str(),
                format!("--export-{}", cli.export_mode).as_str(),
                export_preset.name.as_str(),
                tmp_dir_path.join(executable_file_name).to_str().unwrap(),
            ])
            .stderr(Stdio::inherit())
            .output()
    }

    fn get_exported_files(output_folder: &str) -> Result<Vec<PathBuf>, Box<std::io::Error>> {
        Ok(
            std::fs::read_dir(PathBuf::from(output_folder).join(TMP_DIR_NAME))?
                .map(|path| path.unwrap().path())
                .collect::<Vec<PathBuf>>(),
        )
    }

    fn package(&self, cli: &Cli) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub enum Error {
    IniError(ini::Error),
    ProjectNameNotFound,
    ProjectIconNotFound,
}

#[derive(Clone)]
pub struct ExportPreset {
    name: String,
    platform: String,
}

pub fn export(cli: &Cli) -> Result<(), Error> {
    let project_configuration = match get_project_configuration(&cli.project_path) {
        Ok(ini) => ini,
        Err(e) => return Err(Error::IniError(e)),
    };

    let project_name: String = match get_project_name(&project_configuration) {
        Some(name) => name.replace("?", ""),
        None => return Err(Error::ProjectNameNotFound),
    };

    let project_icon_path: PathBuf = PathBuf::from(&cli.project_path).join(
        match get_project_icon_relative_path(&project_configuration) {
            Some(path) => path,
            None => return Err(Error::ProjectIconNotFound),
        },
    );

    let project_version = get_project_version(&project_configuration);

    println!("Project version: {:?}", &project_version);

    let export_presets = match get_export_presets(&cli.presets, &cli.project_path) {
        Ok(vec) => vec,
        Err(e) => return Err(Error::IniError(e)),
    };
    for export_preset in export_presets {
        match export_preset.platform.as_str() {
            "Windows Desktop" => {
                let windows_conf = windows::Conf {
                    project_name: project_name.to_owned(),
                    project_version: project_version.to_owned().unwrap_or("".to_owned()),
                };
                let windows_exporter = windows::WindowsExporter {
                    conf: windows_conf,
                    preset: export_preset.clone(),
                };
                let windows_result = windows_exporter.package(&cli);
                if windows_result.is_err() {
                    eprintln!(
                        "Error exporting to windows: {:?}",
                        windows_result.err().unwrap()
                    );
                } else {
                    println!("Finishes expoting Windows preset {}", &export_preset.name);
                }
            }
            "Linux/X11" => {
                let linux_conf = linux::Conf {
                    project_name: project_name.to_owned(),
                    project_version: project_version.to_owned().unwrap_or("".to_owned()),
                    project_icon: project_icon_path.to_owned(),
                };
                let linux_exporter = linux::LinuxExporter {
                    conf: linux_conf,
                    preset: export_preset.clone(),
                };
                let linux_result = linux_exporter.package(&cli);
                if linux_result.is_err() {
                    eprintln!(
                        "Error exporting to linux: {:?}",
                        linux_result.err().unwrap()
                    );
                } else {
                    println!("Finishes expoting Linux preset {}", &export_preset.name);
                }
            }
            _ => {
                eprintln!("Invalid platform {}", export_preset.platform)
            }
        }
    }

    Ok(())
}

fn get_export_presets(
    cli_presets: &Vec<String>,
    project_path: &str,
) -> Result<Vec<ExportPreset>, ini::Error> {
    let mut vec = Vec::new();

    let presets_path = PathBuf::from(project_path).join("export_presets.cfg");
    let presets_file = Ini::load_from_file(presets_path)?;
    let mut i: u8 = 0;
    while let Some(section) = presets_file.section(Some(format!("preset.{}", i))) {
        println!("Found preset.{}", i);
        let name = section.get("name").unwrap_or("no name");
        if !cli_presets.is_empty() && !cli_presets.contains(&name.to_owned()) {
            println!("Skipping {} preset", name);
            i += 1;
            continue;
        }
        let platform = section.get("platform").unwrap_or("no platform");

        vec.push(ExportPreset {
            name: name.to_owned(),
            platform: platform.to_owned(),
        });

        i += 1;
    }

    Ok(vec)
}

fn get_project_name(project_configuration: &Ini) -> Option<String> {
    Some(
        project_configuration
            .section(Some("application"))?
            .get("config/name")?
            .to_owned(),
    )
}

fn get_project_icon_relative_path(project_configuration: &Ini) -> Option<String> {
    Some(
        project_configuration
            .section(Some("application"))?
            .get("config/icon")?
            .strip_prefix("res://")?
            .to_owned(),
    )
}

fn get_project_version(project_configuration: &Ini) -> Option<String> {
    Some(
        project_configuration
            .section(Some("global"))?
            .get("version")?
            .to_owned(),
    )
}

fn get_project_configuration(project_path: &str) -> Result<Ini, ini::Error> {
    let mut godot_project_file_path = PathBuf::from(project_path);
    godot_project_file_path.push("project.godot");
    /*  if !godot_project_file_path.is_file() {
        std::io::Error::
    } */

    Ini::load_from_file(godot_project_file_path)
}
