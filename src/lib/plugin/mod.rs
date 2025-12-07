pub mod fetcher;
pub mod parser;

use std::{marker::PhantomData, path::PathBuf};

use color_eyre::eyre::Result;

trait PluginType {}

struct Fetcher {}
impl PluginType for Fetcher {}

pub struct Plugin<T>
where
    T: PluginType,
{
    path: PathBuf,
    name: String,
    // TODO: Define the input/output contract format for the plugin, to allow
    // them to declare (using the `info` subcommand) what their input and output
    // schema is.
    input: String,
    output: String,
    // TODO: The plugin manifest should include dependency hashes as well
    _type: PhantomData<T>,
}
