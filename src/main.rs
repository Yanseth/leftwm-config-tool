extern crate toml;
extern crate serde_derive;

use serde_derive::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
struct Config {
    modkey: String,
    keybind: Vec<Keybind>,
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Deserialize)]
struct Keybind {
    command: String,
    value: Option<String>,
    modifier: Option<Vec<String>>,
    key: String,
}

fn main() {
    let config: Config = read_config();
    println!("{:?}", config);
    assert_eq!(config.modkey, "Mod4");
}

fn read_config() -> Config {
    toml::from_str(r#"
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

    "#).unwrap()
}
