use crate::csv_parser::Entry;
use colored::Colorize;
use std::error::Error;
use std::fs;
use typst::foundations::{Dict, IntoValue, Str};
use typst_as_lib::{TypstEngine, typst_kit_options::TypstKitFontOptions};
use typst_pdf;

pub struct D(pub Dict);

impl From<Entry> for D {
    fn from(value: Entry) -> Self {
        let dict: Dict = value
            .into_iter()
            .map(|(k, v)| (Str::from(k), v.into_value()))
            .collect();
        D(dict)
    }
}

pub fn render(template_path: String, data: Entry) -> Result<Vec<u8>, Box<dyn Error>> {
    let template_file =
        fs::read_to_string(template_path).expect(&*"could not read template file".red());

    let template = TypstEngine::builder()
        .main_file(template_file)
        .search_fonts_with(TypstKitFontOptions::default())
        .build();

    let dict: Dict = D::from(data).0;

    let doc = template
        .compile_with_input(dict)
        .output
        .expect(&*"got an error while compiling the template".red());

    let options = Default::default();
    let output = typst_pdf::pdf(&doc, &options)
        .expect(&*"got an error while converting document to pdf".red());

    Ok(output)
}
