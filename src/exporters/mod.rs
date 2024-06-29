use std::path::PathBuf;

use clap::ValueEnum;
use ini::Ini;

use crate::cli::Cli;

mod linux;
mod windows;

#[derive(Debug, Clone, ValueEnum)]
pub enum ExportMode {
    Debug,
    Release,
}

impl std::fmt::Display for ExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportMode::Debug => write!(f, "debug"),
            ExportMode::Release => write!(f, "release"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IniError(ini::Error),
    ProjectNameNotFound,
    ProjectIconNotFound,
}

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
                    output_folder: cli.output_folder.to_owned(),
                    project_path: cli.project_path.to_owned(),
                    godot_path: cli.godot_path.to_owned(),
                    project_name: project_name.to_owned(),
                    project_version: project_version.to_owned().unwrap_or("".to_owned()),
                    compress: cli.compress,
                };
                let windows_result =
                    windows::export(&cli.export_mode, windows_conf, &export_preset);
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
                    output_folder: cli.output_folder.to_owned(),
                    project_path: cli.project_path.to_owned(),
                    godot_path: cli.godot_path.to_owned(),
                    project_name: project_name.to_owned(),
                    project_version: project_version.to_owned().unwrap_or("".to_owned()),
                    project_icon: project_icon_path.to_owned(),
                    compress: cli.compress,
                };
                let linux_result = linux::export(&cli.export_mode, linux_conf, &export_preset);
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
