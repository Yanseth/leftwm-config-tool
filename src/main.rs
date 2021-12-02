extern crate serde_derive;
extern crate toml;
extern crate html_parser;
extern crate new_string_template;

use std::path::PathBuf;
use std::{fs, io};
use std::collections::HashMap;

use new_string_template::template::Template;
use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Config {
    modkey: Option<String>,
    keybind: Option<Vec<Keybind>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Keybind {
    command: String,
    value: Option<String>,
    modifier: Vec<String>,
    key: String,
}

#[allow(unused)]
fn main() -> io::Result<()> {
    let mut output = String::new();
    let entries = get_config_files("config")?;
    let cols = 3;
    let mut col = 0;
    initalize_output(&mut output);
    for entry in entries.iter() {
        let file_name = entry.file_name();
        let content = read_file(entry)?;
        let config = parse_config(&content);
    }
    finalize_output(&mut output);
    Ok(())
}

#[allow(dead_code, unused)]
fn add_row(output: &mut String) {
    output.push_str("\n<div class=\"row justify-content-center\">")
}

#[allow(dead_code, unused)]
fn add_col(output: &mut String) {
    output.push_str("\n<div class=\"col\">")
}

#[allow(dead_code)]
fn close_div(output: &mut String) {
    output.push_str("\n</div>")
}

#[allow(dead_code, unused)]
fn add_table(output: &mut String, file_path: String, config: &Config)-> io::Result<()> {
    let start = file_path.rfind("/").unwrap(); // The start of the file name
    let file_name = &file_path[start..];
    let h2_str = "\n<h2>{heading}</h2>"; 
    let data = {
        let mut map = HashMap::new();
        map.insert("heading", file_name);
        if file_name == "base" {
            map.insert("modkey", config.modkey.as_ref().unwrap());
        }
        map
    };

    let h2_templ = Template::new(h2_str);
    output.push_str(&h2_templ.render(&data).expect("Expected Result to be OK"));
    if file_name == "base" {
        // add base table
    } else {
        
    }

    Ok(())
}

fn initalize_output(output: &mut String) {
    output.push_str(r#"<!doctype html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>Leftwm Shortcuts</title>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
            </head>
            <body>
                <h1>LeftWM Shortcuts</h1>
                <div class="container">"#)
}

fn finalize_output(output: &mut String) {
    output.push_str(r#"
                </div>
                <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-1BmE4kWBq78iYhFldvKuhfTAU6auU8tT94WrHftjDbrCEXSU1oBoqyl2QvZ6jIW3" crossorigin="anonymous">
            </body>
        </html>"#)
}

fn parse_config(conf: &str) -> Config {
    toml::from_str(conf).unwrap()
}

fn get_config_files(dir: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

fn read_file(file: &PathBuf) -> io::Result<String> {
    let content = fs::read_to_string(file)?;
    Ok(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_parser::Dom;

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
modifier = []
key = "Return"
    "#;

    #[test]
    fn test_parse_config() {
        let config: Config = parse_config(CONF);
        assert_eq!(config.modkey.unwrap(), "Mod4");
        assert_eq!(config.keybind.unwrap().len(), 2)
    }
    #[test]
    fn test_get_config_files() -> io::Result<()> {
        let entries = get_config_files("config")?;
        let mut found = false;
        let base = PathBuf::from("config/base.toml");
        for entry in entries.iter() {
            if &base == entry {
                found = true;
                break;
            }
        }
        assert!(found);
        Ok(())
    }
    #[test]
    fn test_read_file() -> io::Result<()> {
        let file = PathBuf::from("config/test.toml");
        let file_content = read_file(&file)?;
        let config = parse_config(&file_content);
        assert_eq!(config.modkey.unwrap(), "Mod4");
        assert_eq!(config.keybind.unwrap().len(), 2);
        Ok(())
    }
    #[test]
    fn test_init_output() {
        let mut output = String::new();
        initalize_output(&mut output);  
        assert_eq!("<!doctype html>", &output[0..15]);
        assert!(! Dom::parse(&output.to_string()).is_ok())
    }
    #[test]
    fn test_finalize_output() {
        let mut output = String::new();
        initalize_output(&mut output);  
        finalize_output(&mut output);
        let start = output.len() - 7;
        assert_eq!("</html>", &output[start..]);
        assert!(! Dom::parse(&output.to_string()).is_ok())
    }
    #[test]
    fn test_init_finalize_output_valid_html() {
        let mut output = String::new();
        initalize_output(&mut output);  
        finalize_output(&mut output);
        assert!(Dom::parse(&output.to_string()).is_ok())
    }
}
