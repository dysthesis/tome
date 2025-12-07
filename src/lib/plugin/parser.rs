use color_eyre::eyre::Result;

use crate::plugin::{Plugin, PluginType};

struct Parser {}
impl PluginType for Parser {}

pub struct ParsedIdentifier(String);

impl Plugin<Parser> {
    pub fn parse(identifier: &str) -> Result<ParsedIdentifier> {
        todo!()
    }
}
