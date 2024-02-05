use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub enum Error {
    GodotCommandError,
}

pub struct Conf {
    pub project_path: String,
    pub godot_path: String,
    pub output_folder: String,
    pub project_name: String,
}

pub fn export(conf: Conf) -> Result<(), Error> {
    let godot_command = Command::new(conf.godot_path)
        .args([
            "--headless",
            "--path",
            conf.project_path.as_str(),
            "--export-release",
            "Windows Desktop",
            PathBuf::from(conf.output_folder)
                .join(Path::new(conf.project_name.as_str()))
                .to_str()
                .unwrap(),
        ])
        .spawn();

    match godot_command {
        Ok(mut child) => {
            if child.wait().is_err() {
                return Err(Error::GodotCommandError);
            }
        }
        Err(_) => return Err(Error::GodotCommandError),
    };

    Ok(())
}
