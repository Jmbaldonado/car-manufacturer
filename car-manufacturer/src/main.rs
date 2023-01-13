#![deny(clippy::all)]

use std::env;
use tokio::runtime::Runtime;

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
 async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }

    let keyword = &args[1];

     let rt = Runtime::new();

     let client = reqwest::Client::new();
         let res = client.get(API_URL).send().await?.json::<serde_json::Value>().await?;

         let manufacturer_json = res.as_object().unwrap().iter().find(|key, _| key == &"Results").unwrap().1.as_array().unwrap().iter();
    Ok(())
    println!("Hello, world!");
}
