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
    fn main() {
    println!("Hello, world!");
    println!("This is my first commit!");
}
