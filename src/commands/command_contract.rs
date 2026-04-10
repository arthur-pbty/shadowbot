#[derive(Clone, Copy)]
pub struct CommandMetadata {
    pub name: &'static str,
    pub category: &'static str,
    pub allow_in_dm: bool,
    pub default_permission: u8,
    pub params: &'static str,
    pub description: &'static str,
    pub examples: &'static [&'static str],
    pub default_aliases: &'static [&'static str],
}

pub trait CommandSpec: Send + Sync {
    fn metadata(&self) -> CommandMetadata;
}
