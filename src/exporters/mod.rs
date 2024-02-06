use std::path::PathBuf;

use ini::Ini;

pub mod windows;

pub enum Error {
    IniError(ini::Error),
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

    println!("Project version: {}", get_version(&project_configuration));

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
                    project_name: get_project_name(&project_configuration),
                };
                let windows_result = windows::export(windows_conf, export_preset);
                if windows_result.is_err() {
                    eprintln!("Error exporting to windows");
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

fn get_project_name(project_configuration: &Ini) -> String {
    project_configuration
        .section(Some("application"))
        .unwrap()
        .get("config/name")
        .unwrap()
        .to_lowercase()
        .replace(" ", "_")
}

fn get_version(project_configuration: &Ini) -> String {
    project_configuration
        .section(Some("global"))
        .unwrap()
        .get("version")
        .unwrap()
        .to_string()
}

fn get_project_configuration(project_path: &str) -> Result<Ini, ini::Error> {
    let mut godot_project_file_path = PathBuf::from(project_path);
    godot_project_file_path.push("project.godot");
    /*  if !godot_project_file_path.is_file() {
        std::io::Error::
    } */

    Ini::load_from_file(godot_project_file_path)
}
