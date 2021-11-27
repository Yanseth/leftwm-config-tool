extern crate serde_derive;
extern crate toml;

use std::{fs, io};
use std::path::PathBuf;

use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Config {
    modkey: String,
    keybind: Vec<Keybind>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Keybind {
    command: String,
    value: Option<String>,
    modifier: Option<Vec<String>>,
    key: String,
}

fn main() -> io::Result<()> {
    Ok(())
}

#[allow(dead_code)]
fn parse_config(conf: &str) -> Config {
    toml::from_str(conf).unwrap()
}

#[allow(dead_code)]
fn get_config_files(dir: &str, ) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

#[allow(dead_code)]
fn read_file(file: &PathBuf) -> io::Result<String> {
    let content = fs::read_to_string(file)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONF: &str = r#"
modkey = "Mod4"
mousekey = "Mod4"
tags = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
layouts = ["MainAndVertStack", "MainAndHorizontalStack", "MainAndDeck", "GridHorizontal", "EvenHorizontal", "EvenVertical", "Fibonacci", "CenterMain", "CenterMainBalanced", "Monocle", "RightWiderLeftStack", "LeftWiderRightStack"]
disable_current_tag_swap = false
focus_behaviour = "Sloppy"
focus_new_windows = true

[[keybind]]
command = "Execute"
value = "rofi -show run"
modifier = ["modkey"]
key = "p"

[[keybind]]
command = "Execute"
value = "alacritty"
modifier = ["modkey", "Shift"]
key = "Return"
    "#;

    #[test]
    fn test_parse_config() {
        let config: Config = parse_config(CONF);
        assert_eq!(config.modkey, "Mod4");
        assert_eq!(config.keybind.len(), 2)
    }
    
    #[test]
    fn test_get_config_files() -> io::Result<()> {
        let entries = get_config_files("config")?;
        let mut found  = false;
        let base = PathBuf::from("config/base.toml");
        for entry in entries.iter() {
            if &base == entry {
                found = true;
                break
            }
        }
        assert!(found);
        Ok(())
    }
    
    #[test]
    fn test_read_file() -> io::Result<()> {
        let file = PathBuf::from("config/test.toml");
        let file_content = read_file(&file)?;
        let config = parse_config(&file_content.to_string());
        assert_eq!(config.modkey, "Mod4");
        assert_eq!(config.keybind.len(), 2);
        Ok(())
    }
}
