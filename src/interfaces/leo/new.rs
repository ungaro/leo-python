use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::IntoPyDict;


use leo_lang::{
    commands::{Command, Network, ALEO_CLI_COMMAND},
    context::Context,
};
use leo_errors::{CliError, PackageError, Result};
use leo_package::{build::BUILD_DIRECTORY_NAME, package::Package};
use snarkvm::file::AleoFile;

use snarkvm::cli::New as AleoNew;



#[pyclass]
pub struct PyNew {
    name: String,
    debug: bool,
    path: Option<String>,
    quiet: bool,
}

#[pymethods]
impl PyNew {
    #[new]
    #[args(
        NAME,
        d = "false",
        path = "None",
        q = "false"
    )]
    fn new(
        NAME: &str,
        d: bool,
        path: Option<&str>,
        q: bool
    ) -> Self {
        PyNew {
            name: NAME.to_string(),
            debug: d,
            path: path.map(String::from),
            quiet: q,
        }
  // Call the `aleo new` command from the Aleo SDK.
  let command =
  AleoNew::try_parse_from([ALEO_CLI_COMMAND, &self.name]).map_err(CliError::failed_to_parse_aleo_new)?;
let result = command.parse().map_err(CliError::failed_to_execute_aleo_new)?;

// todo: modify the readme file to recommend building with `leo build`.

// Log the output of the `aleo new` command.
tracing::info!("{}", result);

// Derive the program directory path.
let mut package_path = context.dir()?;
package_path.push(&self.name);

// Initialize the Leo package in the directory created by `aleo new`.
Package::initialize(&self.name, &package_path)?;

// Change the cwd to the Leo package directory to compile aleo files.
std::env::set_current_dir(&package_path)
  .map_err(|err| PackageError::failed_to_set_cwd(package_path.display(), err))?;

// Open the program manifest.
let manifest = context.open_manifest()?;

// Create a path to the build directory.
let mut build_directory = package_path.clone();
build_directory.push(BUILD_DIRECTORY_NAME);

// Write the Aleo file into the build directory.
AleoFile::create(&build_directory, manifest.program_id(), true)
  .map_err(PackageError::failed_to_create_aleo_file)?;

// build_aleo_file.push(AleoFile::<Network>::main_file_name());
//
// println!("{}", build_aleo_file.display());
//
//
// std::fs::File::create(build_aleo_file).map_err()
// aleo_file.write_to(&build_aleo_file).map_err(PackageError::failed_to_write_aleo_file)?;

// Open the `main.aleo` file path.
let aleo_file = AleoFile::open(&package_path, manifest.program_id(), true)
  .map_err(PackageError::failed_to_open_aleo_file)?;

let mut aleo_file_path = package_path.clone();
aleo_file_path.push(AleoFile::<Network>::main_file_name());

// Remove the Aleo file from the package directory.
aleo_file.remove(&aleo_file_path).map_err(PackageError::failed_to_remove_aleo_file)?;

Ok(())








    }

    // Other methods go here
}

#[pymodule]
fn new(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNew>()?;
    Ok(())
}