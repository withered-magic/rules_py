use std::path::PathBuf;

use clap::Parser;
use miette::Context;

use py;

#[derive(Parser, Debug)]
struct VenvArgs {
    /// Source Python interpreter path to symlink into the venv.
    #[arg(long)]
    python: PathBuf,

    /// Destination path of the venv.
    #[arg(long)]
    location: PathBuf,

    /// Python version string to use, eg 3.8.12
    /// Must be seperated by dots.
    #[arg(long)]
    python_version: String,

    /// Path to a .pth file to add to the site-packages of the generated venv.
    #[arg(long)]
    pth_file: PathBuf,

    /// Prefix to append to each .pth path entry.
    #[arg(long)]
    pth_entry_prefix: Option<String>,

    /// Path to the current Bazel workspace.
    /// Corresponds to the `BUILD_WORKSPACE_DIRECTORY` environment variable.
    #[arg(long, env = "BUILD_WORKSPACE_DIRECTORY")]
    build_workspace_directory: Option<PathBuf>,

    /// Additional paths relative to the current Bazel workspace to be added as .pth path entries.
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    additional_workspace_paths: Option<Vec<String>>,
}

fn venv_cmd_handler(args: VenvArgs) -> miette::Result<()> {
    let pth_file = py::PthFile::new(&args.pth_file, args.pth_entry_prefix);
    py::create_venv(
        &args.python,
        &args.python_version,
        &args.location,
        Some(pth_file),
        args.build_workspace_directory.as_deref(),
        args.additional_workspace_paths
            .as_ref()
            .map(|s| s.as_slice()),
    )
}

fn main() -> miette::Result<()> {
    let args = VenvArgs::parse();
    venv_cmd_handler(args).wrap_err("Unable to run command:")
}
