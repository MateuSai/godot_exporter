use std::path::PathBuf;

pub mod windows;

pub fn export() {
    windows::hi();
}

pub fn get_version(project_path: PathBuf) -> String {
    let mut godot_project_file_path = project_path;
    godot_project_file_path.push("project.godot");
    if !godot_project_file_path.is_file() {
        eprintln!("Can't find project.godot file");
        std::process::exit(2);
    }
    let godot_project = ini::Ini::load_from_file(godot_project_file_path).unwrap();

    godot_project
        .section(Some("global"))
        .unwrap()
        .get("version")
        .unwrap()
        .to_string()
}
