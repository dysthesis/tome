use color_eyre::eyre::Result;

use crate::plugin::{Plugin, PluginType, parser::ParsedIdentifier};

struct Fetcher {}
impl PluginType for Fetcher {}

impl Plugin<Fetcher> {
    pub fn parse(identifier: ParsedIdentifier) -> Result<String> {
        todo!()
    }
}
