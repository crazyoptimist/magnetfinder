use std::env;
use std::fs;
use std::io::{self, Write};

use config::{Config, ConfigError, File};
use directories::ProjectDirs;

use crate::Settings;

impl Settings {
    pub fn fetch() -> Result<Settings, ConfigError> {
        let builder = Config::builder();

        let builder = if let Some(proj_dirs) = ProjectDirs::from("", "", "magnetfinder") {
            let config_path = proj_dirs.config_dir();
            let mut config_path = config_path.to_path_buf();
            config_path.push("Settings.toml");

            builder.add_source(File::from(config_path))
        } else {
            eprintln!("Error finding project config directory, falling back to executable path");
            match env::current_exe() {
                Ok(mut exe_path) => {
                    exe_path.pop();
                    exe_path.push("Settings.toml");
                    builder.add_source(File::from(exe_path))
                }
                Err(_) => builder.add_source(File::with_name("Settings")),
            }
        };

        let s = builder.build()?;

        let default_proxy = s
            .get::<String>("default_proxy")
            .unwrap_or_else(|_| String::from(""));

        Ok(Settings { default_proxy })
    }

    pub fn generate_settings_file() -> Result<(), io::Error> {
        let mut file;

        if let Some(proj_dirs) = ProjectDirs::from("", "", "magnetfinder") {
            let config_path = proj_dirs.config_dir();
            let mut config_path = config_path.to_path_buf();

            if !config_path.is_dir() {
                fs::create_dir(&config_path)?;
            }

            config_path.push("Settings.toml");

            file = fs::File::create(config_path)?;
        } else {
            eprintln!("Error finding project config directory, falling back to executable path");
            if let Ok(mut exe_path) = env::current_exe() {
                exe_path.pop();
                exe_path.push("Settings.toml");
                file = fs::File::create(exe_path)?;
            } else {
                file = fs::File::create("Settings.toml")?;
            }
        }

        file.write_all(
            b"# setting a default proxy allows you to tunnel all scraping from
# the pirate bay through this set proxy by default. If using a socks5
# proxy, format ip like so: socks5://192.168.1.1:9000
default_proxy = \"\"",
        )?;

        Ok(())
    }
}
