extern crate serde_derive;
extern crate toml;
extern crate html_parser;
extern crate new_string_template;

use std::path::PathBuf;
use std::{fs, io};
use std::collections::HashMap;

use new_string_template::template::Template;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    modkey: Option<String>,
    mousekey: Option<String>,
    keybind: Option<Vec<Keybind>>,
}

#[derive(Deserialize)]
struct Keybind {
    command: String,
    value: Option<String>,
    modifier: Vec<String>,
    key: String,
}

fn main() -> io::Result<()> {
    let mut output = String::new();
    let entries = get_config_files("config")?;
    initalize_output(&mut output);
    let cols = 2;
    let mut current_col = 0;
    let mut row_open = true;
    for entry in entries.iter() {
        if current_col == 0 {
            add_row(&mut output);
            row_open = true;
        }
        add_col(&mut output);
        current_col += 1;
        let file_name = entry.to_str().unwrap();
        let content = read_file(entry)?;
        let config = parse_config(&content);
        add_table(&mut output, file_name, &config);
        close_div(&mut output); // col
        if current_col % cols == 0 {
            close_div(&mut output); // row
            row_open = false;
            current_col = 0;
        }
    }
    if row_open {
        close_div(&mut output) // close row
    }
    finalize_output(&mut output);
    fs::write("keybinds.html", output)?;
    Ok(())
}

fn add_row(output: &mut String) {
    output.push_str("\n<div class=\"row justify-content-center\">")
}

fn add_col(output: &mut String) {
    output.push_str("\n<div class=\"col\">")
}

fn close_div(output: &mut String) {
    output.push_str("\n</div>")
}

fn add_table(output: &mut String, file_path: &str, config: &Config) {
    let start = file_path.rfind("/").unwrap() + 1; // The start of the file name
    let file_name = &file_path[start..];
    let h2_str = "\n<h2>File \"{heading}\"</h2>"; 
    let h2_templ = Template::new(h2_str);
    let data = {
        let mut map = HashMap::new();
        map.insert("heading", file_name);
        map
    };

    output.push_str(&h2_templ.render(&data).expect("Expected Result to be OK"));
    if file_name == "base.toml" {
        output.push_str("\n<h3>Basic Config</h3>");
        initalize_table(output);
        add_table_head_row(output, &["Setting", "Value"]);
        add_table_data_row(output, &["modkey", config.modkey.as_ref().unwrap()]);
        add_table_data_row(output, &["mousekey", config.mousekey.as_ref().unwrap()]);
        finalize_table(output)
    }
    
    output.push_str("\n<h3>Keybinds</h3>");
    initalize_table(output);
    add_table_head_row(output, &["Modifier", "Key", "Command[: Value]"]);
    let empty_vec: Vec<Keybind> = Vec::new(); // for if no keybinds present
    for keybind in config.keybind.as_ref().unwrap_or(&empty_vec) {
        let mut command = String::new();
        command.push_str(&keybind.command);
        if keybind.command == "Execute" {
            command.push_str(": ");
            command.push_str("<b>&lt;");
            command.push_str(&keybind.value.as_ref().unwrap());
            command.push_str("&gt;</b>");
        }
        add_table_data_row(output, &[
            &keybind.modifier.join(", ").to_string(),
            &keybind.key,
            &command.to_string(),
        ]);
    }
    
    finalize_table(output)
}

fn initalize_table(output: &mut String) {
    //output.push_str("\n<table class=\"table table-dark table-borderless table-sm\">");
    output.push_str("\n<table class=\"table table-dark table-striped\">");
}

fn add_table_head_row(output: &mut String, col_data: &[&str]) {
    open_tag(output, "thead");
    open_tag(output, "tr");
    add_table_row_headers(output, col_data);
    close_tag(output, "tr");
    close_tag(output, "thead");
}

fn add_table_data_row(output: &mut String, col_data: &[&str]) {
    open_tag(output, "tbody");
    open_tag(output, "tr");
    add_table_row_data(output, col_data);
    close_tag(output, "tr");
    close_tag(output, "tbody");
}

fn add_table_row_headers(output: &mut String, col_data: &[&str]) {
    for col in col_data {
        output.push_str("\n<th scope=\"col\">");
        output.push_str(col);
        output.push_str("</th>")
    }
}

fn add_table_row_data(output: &mut String, col_data: &[&str]) {
    for col in col_data {
        output.push_str("\n<td>");
        output.push_str(col);
        output.push_str("</td>")
    }
}

fn open_tag(output: &mut String, tag: &str) {
    output.push_str("\n<");
    output.push_str(tag);
    output.push_str(">");
}

fn  close_tag(output: &mut String, tag: &str) {
    output.push_str("\n</");
    output.push_str(tag);
    output.push_str(">");
}

fn finalize_table(output: &mut String) {
    output.push_str("\n</table>")
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
