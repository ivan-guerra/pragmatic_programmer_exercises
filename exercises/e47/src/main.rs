//! # Astronauts in Space Tracker
//!
//! This module implements an application that retrieves and displays information about
//! the current astronauts in space using the Open Notify API.
//!
//! ## Features
//!
//! - **Real-time Data**: Fetches current information about astronauts in space
//! - **Sorted Display**: Presents astronauts sorted by last name
//! - **Formatted Output**: Shows data in a clean, tabular format with proper alignment
//! - **Spacecraft Information**: Includes details about which spacecraft each astronaut is on
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Astronaut {
    name: String,
    craft: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SpaceInfo {
    people: Vec<Astronaut>,
    number: u32,
    message: String,
}

fn get_astronauts() -> anyhow::Result<SpaceInfo> {
    let url = "http://api.open-notify.org/astros.json";
    let response = reqwest::blocking::get(url)?.json::<SpaceInfo>()?;
    Ok(response)
}

fn print_astronauts(space_info: &SpaceInfo) {
    let mut sorted_people = space_info.people.clone();
    sorted_people.sort_by(|a, b| {
        let a_last = a
            .name
            .split_whitespace()
            .skip(a.name.len() - 1)
            .collect::<Vec<&str>>()
            .join(" ");
        let b_last = b
            .name
            .split_whitespace()
            .skip(b.name.len() - 1)
            .collect::<Vec<&str>>()
            .join(" ");
        a_last.cmp(&b_last)
    });

    let name_width = space_info
        .people
        .iter()
        .map(|a| a.name.len())
        .max()
        .unwrap_or(0)
        + 1;
    let craft_width = space_info
        .people
        .iter()
        .map(|a| a.craft.len())
        .max()
        .unwrap_or(0)
        + 1;
    println!(
        "{:<width$} | {:<craft_width$}",
        "Name",
        "Craft",
        width = name_width,
        craft_width = craft_width
    );
    println!(
        "{:-<width$} | {:-<craft_width$}",
        "",
        "",
        width = name_width,
        craft_width = craft_width
    );
    for astronaut in &sorted_people {
        println!(
            "{:<width$} | {:<craft_width$}",
            astronaut.name,
            astronaut.craft,
            width = name_width,
            craft_width = craft_width
        );
    }
}

fn main() -> anyhow::Result<()> {
    let space_info = get_astronauts()?;

    print_astronauts(&space_info);

    Ok(())
}
