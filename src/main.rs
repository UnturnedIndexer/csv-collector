mod cli;

use std::path::Path;

use clap::Parser as _;
use cli::Cli;

use csv_collector::basic_parser::item::Item;
use csv_collector::basic_parser::Parser as _;

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

    for path in paths {
        if let Ok(item) = Item::parse(path) {
            writer.serialize(item)?;
        }
    }

    Ok(())
}
