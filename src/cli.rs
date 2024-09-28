//! The clap declarative command line interface

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    author,
    about,
    arg_required_else_help(true),
    subcommand_required(true),
)]
pub struct MainArguments {
    #[command(subcommand)]
    pub subcommand: MainSubcommand,
}

#[derive(Subcommand)]
pub enum MainSubcommand {
    Clean(CleanPackageAction),
    Review(ReviewPackageAction),
    Sync(SyncPackageAction),
    Unmanaged(UnmanagedPackageAction),
}

#[derive(Args)]
#[command(visible_alias("c"))]
/// remove unmanaged packages
pub struct CleanPackageAction {
    #[arg(long)]
    /// do not ask for any confirmation
    pub no_confirm: bool,
}

#[derive(Args)]
#[command(visible_alias("r"))]
/// review unmanaged packages
pub struct ReviewPackageAction {}

#[derive(Args)]
#[command(visible_alias("s"))]
/// install packages from groups
pub struct SyncPackageAction {
    #[arg(long)]
    /// do not ask for any confirmation
    pub no_confirm: bool,
}

#[derive(Args)]
#[command(visible_alias("u"))]
/// show explicitly installed packages not managed by pacdef
pub struct UnmanagedPackageAction {}
