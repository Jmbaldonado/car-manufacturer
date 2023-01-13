#![deny(clippy::all)]

use std::env;

const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/GetAllManufacturers?format=json";

struct Manufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or_default().contains(needle) || self.common_name.unwrap_or_default()
            .contains(needle) || self.country.unwrap_or_default().contains(needle)
    }
}

impl<'a> Manufacturer<'a> {
    fn description(&self) -> String {
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();

        format!("\tName: {}\n\tCommon Name: {}\n\tCountry: {}", name, common_name, country)
    }
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }

    let keyword = &args[1];
    Ok(())
    println!("Hello, world!");
    println!("This is my first commit!");
}
