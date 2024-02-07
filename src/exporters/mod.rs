use std::path::PathBuf;

use ini::Ini;

mod linux;
mod windows;

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

pub fn export(project_path: String, godot_path: String, output: String) -> Result<(), Error> {
    let project_configuration = match get_project_configuration(&project_path) {
        Ok(ini) => ini,
        Err(e) => return Err(Error::IniError(e)),
    };

    let project_name: String = match get_project_name(&project_configuration) {
        Some(name) => name,
        None => return Err(Error::ProjectNameNotFound),
    };

    let project_icon_path: PathBuf = PathBuf::from(&project_path).join(
        match get_project_icon_relative_path(&project_configuration) {
            Some(path) => path,
            None => return Err(Error::ProjectIconNotFound),
        },
    );

    let project_version = get_project_version(&project_configuration);

    println!("Project version: {:?}", project_version);

    let export_presets = match get_export_presets(&project_path) {
        Ok(vec) => vec,
        Err(e) => return Err(Error::IniError(e)),
    };
    for export_preset in export_presets {
        match export_preset.platform.as_str() {
            "Windows Desktop" => {
                let windows_conf = windows::Conf {
                    output_folder: output.to_owned(),
                    project_path: project_path.to_owned(),
                    godot_path: godot_path.to_owned(),
                    project_name: project_name.to_owned(),
                };
                let windows_result = windows::export(windows_conf, export_preset);
                if windows_result.is_err() {
                    eprintln!(
                        "Error exporting to windows: {:?}",
                        windows_result.err().unwrap()
                    );
                }
            }
            "Linux/X11" => {
                let linux_conf = linux::Conf {
                    output_folder: output.to_owned(),
                    project_path: project_path.to_owned(),
                    godot_path: godot_path.to_owned(),
                    project_name: project_name.to_owned(),
                };
                let linux_result = linux::export(linux_conf, export_preset);
                if linux_result.is_err() {
                    eprintln!(
                        "Error exporting to linux: {:?}",
                        linux_result.err().unwrap()
                    );
                }
            }
            _ => {
                eprintln!("Invalid platform {}", export_preset.platform)
            }
        }
    }

    Ok(())
}

fn get_export_presets(project_path: &str) -> Result<Vec<ExportPreset>, ini::Error> {
    let mut vec = Vec::new();

    let presets_path = PathBuf::from(project_path).join("export_presets.cfg");
    let presets_file = Ini::load_from_file(presets_path)?;
    let mut i: u8 = 0;
    while let Some(section) = presets_file.section(Some(format!("preset.{}", i))) {
        println!("Found preset.{}", i);
        let name = section.get("name").unwrap_or("no name");
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
