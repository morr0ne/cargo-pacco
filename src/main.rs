use std::fs;

use anyhow::Result;
use cargo_metadata::{Metadata, Package};
use clap::{builder::Styles, Parser};
use clap_cargo::style;
use quote::ToTokens;
use tracing::{info, warn};

pub const CARGO_STYLE: Styles = Styles::styled()
    .header(style::HEADER)
    .usage(style::USAGE)
    .literal(style::LITERAL)
    .placeholder(style::PLACEHOLDER)
    .error(style::ERROR)
    .valid(style::VALID)
    .invalid(style::INVALID);

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = CARGO_STYLE)]
enum CargoCli {
    Pacco(PaccoArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct PaccoArgs {}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let CargoCli::Pacco(_args) = CargoCli::parse();

    let mut metadata = cargo_metadata::MetadataCommand::new().exec()?;

    let _root = root_package(&mut metadata).expect("Failed to find a root package");

    for package in metadata.packages {
        let name = package.name;

        if let Some(target) = package
            .targets
            .into_iter()
            .find(|t| t.is_lib() || t.is_proc_macro())
        {
            info!("Parsing package {name}");

            let path = target.src_path;

            let main_module = fs::read_to_string(path)?;

            let parsed = syn::parse_file(&main_module)?;

            fs::write(
                format!("packed/{name}"),
                parsed.to_token_stream().to_string(),
            )?;
        } else {
            warn!("Skipping package: {name}");
        }
    }

    // dbg!(parsed);

    Ok(())
}

pub fn root_package(metadata: &mut Metadata) -> Option<Package> {
    match &metadata.resolve {
        Some(resolve) => {
            // if dependencies are resolved, use Cargo's answer
            let root = resolve.root.as_ref()?;
            metadata
                .packages
                .iter()
                .position(|pkg| pkg.id == *root)
                .map(|e| metadata.packages.remove(e))
        }
        None => {
            // if dependencies aren't resolved, check for a root package manually
            let root_manifest_path = metadata.workspace_root.join("Cargo.toml");

            metadata
                .packages
                .iter()
                .position(|pkg| pkg.manifest_path == root_manifest_path)
                .map(|e| metadata.packages.remove(e))
        }
    }
}
