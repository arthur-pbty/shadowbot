#[derive(Clone, Copy)]
pub struct CommandMetadata {
    pub key: &'static str,
    pub command: &'static str,
    pub category: &'static str,
    pub default_permission: u8,
    pub params: &'static str,
    pub summary: &'static str,
    pub description: &'static str,
    pub examples: &'static [&'static str],
    pub alias_source_key: &'static str,
    pub default_aliases: &'static [&'static str],
}

pub trait CommandSpec: Send + Sync {
    fn metadata(&self) -> CommandMetadata;
}
