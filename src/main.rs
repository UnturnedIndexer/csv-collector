mod cli;

use std::path::Path;

use clap::Parser as _;
use cli::Cli;

use csv_collector::parser::Asset;
use csv_collector::parser::Parser as _;

use masterbundle_collector::masterbundle::MasterBundle;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let bundle = MasterBundle::new(args.path)?;
    let paths: Vec<&Path> = bundle.assets.iter().map(|x| x.parent().unwrap()).collect();

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(args.file)?;
    let mut writer = csv::Writer::from_writer(file);

    let mut items: Vec<Asset> = Vec::new();

    for path in paths {
        if let Ok(item) = Asset::parse(path) {
            items.push(item)
        }
    }

    items.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    if args.dedup {
        items.dedup_by(|a, b| a.id == b.id);
    }

    for item in items {
        writer.serialize(item)?;
    }

    Ok(())
}
