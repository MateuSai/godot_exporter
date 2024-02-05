use std::path::PathBuf;

use ini::Ini;

pub mod windows;

pub fn export(project_path: String, godot_path: String, output: String) {
    println!("Project version: {}", get_version(&project_path));

    let windows_conf = windows::Conf {
        output_folder: output,
        project_path: project_path.to_owned(),
        godot_path: godot_path,
        project_name: get_project_name(&project_path),
    };
    let windows_result = windows::export(windows_conf);
    if windows_result.is_err() {
        eprintln!("Error exporting to windows")
    }
}

fn get_project_name(project_path: &str) -> String {
    get_project_configuration(project_path)
        .section(Some("application"))
        .unwrap()
        .get("config/name")
        .unwrap()
        .to_lowercase()
        .replace(" ", "_")
}

pub fn get_version(project_path: &str) -> String {
    get_project_configuration(project_path)
        .section(Some("global"))
        .unwrap()
        .get("version")
        .unwrap()
        .to_string()
}

fn get_project_configuration(project_path: &str) -> Ini {
    let mut godot_project_file_path = PathBuf::from(project_path);
    godot_project_file_path.push("project.godot");
    if !godot_project_file_path.is_file() {
        eprintln!("Can't find project.godot file");
        std::process::exit(2);
    }

    Ini::load_from_file(godot_project_file_path).unwrap()
}
