use std::path::PathBuf;

pub struct Plugin {
    path: PathBuf,
    name: String,
    // TODO: Define the input/output contract format for the plugin, to allow
    // them to declare (using the `info` subcommand) what their input and output
    // schema is.
    input: String,
    output: String,
    // TODO: The plugin manifest should include dependency hashes as well
}

enum Type {
    Fetcher,
    Parser,
}
