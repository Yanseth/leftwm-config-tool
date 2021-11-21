#!/usr/bin/env bash
leftwm_home="$HOME/.config/leftwm"
leftwm_conf="$leftwm_home/config.toml"

# Verify leftwm confgi exists
if [[ ! -d "$leftwm_home" ]]; then
    echo "It appears leftwm is not installed."
    exit 1
fi

# Generate the config file
if [[ -f config/base.toml ]]; then
    output=$(cat "config/base.toml")
    for n in $(ls config/*.toml)
    do
        if [ "$n" != "config/base.toml" ]; then
            file=$(cat "$n")
            output="$output\n\n$file"
        fi
    done
    echo -e "$output" > config.toml
else
    # TODO: Create a base.toml.template that can be used if base does not exist
    echo "You need at least config/base.toml"
    exit 1
fi

# Backup the old file
if [[ -f "$leftwm_conf" ]]; then
    mv "$leftwm_conf" "$leftwm_home/config.toml.prev"
fi

# cp the config over
cp config.toml "$leftwm_home"
