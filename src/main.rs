use clap::{builder::Styles, Parser};
use clap_cargo::style;

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

fn main() {
    let CargoCli::Pacco(_args) = CargoCli::parse();
}
