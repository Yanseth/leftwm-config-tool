use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    modkey: String,
    keybind: Keybind,
}

#[derive(Deserialize)]
struct Keybind {
    command: String,
    value: Option<String>,
    modifier: Option<[String; 5]>,
    key: String,
}

fn main() {
    let config: Config = toml::from_str(r#"
        modkey = 'Mod4'

        [[keybind]]
        command = 'Execute'
        value = 'alacritty'
        modifier = ['modkey', 'Shift']
        key = 'Return'
    "#).unwrap();

    assert_eq!(config.modkey, "Mod4")
}
