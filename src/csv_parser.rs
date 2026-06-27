use std::collections::HashMap;
use std::error::Error;

pub type Entry = HashMap<String, String>;

pub fn get_data(path: String) -> Result<Vec<Entry>, Box<dyn Error>> {
    let rdr = csv::Reader::from_path(path);
    let mut data: Vec<Entry> = Vec::new();
    for result in rdr?.deserialize() {
        let entry: Entry = result?;
        data.push(entry);
    }
    Ok(data)
}
