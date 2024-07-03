use {
  super::*,
  clap::builder::{
    styling::{AnsiColor, Effects},
    Styles,
  },
};

mod package;
mod server;

#[derive(Parser)]
#[command(
  version,
  styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Green.on_default() | Effects::BOLD)
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
    .placeholder(AnsiColor::Cyan.on_default()))
]
pub enum Subcommand {
  Package(package::Package),
  Server(server::Server),
}

impl Subcommand {
  pub fn run(self) -> Result {
    match self {
      Self::Package(package) => package.run(),
      Self::Server(server) => server.run(),
    }
  }
}
