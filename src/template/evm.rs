use super::Templated;

pub struct EvmTemplate {
    pub name: String,
    pub version: String,
    pub init_shell: Vec<String>,
}

impl Templated for EvmTemplate {
    fn name(self: &Self) -> String {
        self.name.clone()
    }
    fn init_cmd_line(self: &Self) -> Vec<String> {
        self.init_shell.clone()
    }
}
