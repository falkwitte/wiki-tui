use dirs;
use ini::Ini;
use std::fs;
use reqwest;

const CONFIG_FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

pub struct Config {
    pub config_path: Option<std::path::PathBuf>,
}

impl Config {
    fn get_config_file(&mut self) -> bool {
        match dirs::config_dir() {
            Some(config_path) => {
                let app_dir_path = config_path.join(APP_CONFIG_DIR);
                let config_file_path = app_dir_path.join(CONFIG_FILE_NAME);

                self.config_path = Some(config_file_path.clone());

                if !app_dir_path.exists() {
                    fs::create_dir(app_dir_path);
                    return false
                }

                if !config_file_path.exists() {
                    return false
                }

                true
            }

            None => {
                panic!("Couldn't find your config directory")
            }
        }
    }

    fn create_config_file(&mut self) {
        let file_url = "https://raw.githubusercontent.com/Builditluc/wiki-tui/stable/config.ini";
        let file_content = reqwest::blocking::get(file_url).unwrap().text().unwrap();

        fs::write(&self.config_path.clone().unwrap(), file_content);
    }
}
