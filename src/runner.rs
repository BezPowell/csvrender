use crate::{Args, template::initialize_template};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use strfmt::strfmt;

#[derive(Debug, Serialize, Deserialize)]
struct Record(HashMap<String, String>);

pub fn process_csv(args: Args) -> Result<()> {
    // Initialize Tera template
    let tera = initialize_template(&args.file_template).context("Error reading template.")?;

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(args.csv)?;
    for (row, result) in rdr.deserialize().enumerate() {
        // Deserialize row into Record
        let record: Record = result.context(format!("Error reading row {row}"))?;

        // Generate output path
        let path = PathBuf::from(
            strfmt(&args.output_path, &record.0)
                .context(format!("Error constructing path for row {row}"))?,
        );
        fs::create_dir_all(path.parent().expect("Error getting path parent"))?;

        // Render template from Record
        let context =
            tera::Context::from_serialize(record).context(format!("Error rendering row {row}"))?;
        let rendered = tera
            .render("template", &context)
            .context(format!("Error rendering row {row}"))?;

        // Print template to file
        fs::write(path, rendered).context(format!("Error writing row {row} to file"))?;
    }

    Ok(())
}
