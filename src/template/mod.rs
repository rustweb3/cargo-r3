mod aptos;
mod evm;
mod solana;
mod sui;

use anyhow::Result;
use evm::EvmTemplate;
use log::info;
use solana::SolanaTemplate;

pub enum AppType {
    Evm,
    Solana,
    Aptos,
    Sui,
    None,
}

pub trait Templated {
    fn name(self: &Self) -> String;
    fn init_cmd_line(self: &Self) -> Vec<String>;
    fn hello(self: &Self) {
        info!("Hello, world! {}", self.name())
    }
    fn init(self: &Self) {
        for cmd in self.init_cmd_line() {
            info!("{}", cmd)
        }
    }
}

impl From<&str> for AppType {
    fn from(s: &str) -> Self {
        match s {
            "evm" => AppType::Evm,
            "solana" => AppType::Solana,
            "aptos" => AppType::Aptos,
            "sui" => AppType::Sui,
            _ => AppType::None,
        }
    }
}

pub fn new_app(name: &str, app_type: AppType) -> Result<()> {
    let app: Box<dyn Templated> = match app_type {
        AppType::Evm => Box::new(EvmTemplate {
            name: name.to_string(),
            version: "0.1".to_string(),
            init_shell: vec!["evm".to_string(), "init".to_string()],
        }),
        AppType::Solana => Box::new(SolanaTemplate {
            name: name.to_string(),
            version: "0.1".to_string(),
            init_shell: vec!["solana".to_string(), "init".to_string()],
        }),
        AppType::Aptos => Box::new(aptos::AptosTemplate {
            name: name.to_string(),
            version: "0.1".to_string(),
            init_shell: vec!["aptos".to_string(), "init".to_string()],
        }),
        AppType::Sui => Box::new(sui::SuiTemplate {
            name: name.to_string(),
            version: "0.1".to_string(),
            init_shell: vec!["sui".to_string(), "init".to_string()],
        }),
        AppType::None => {
            info!("No app type specified");
            return Ok(());
        }
    };

    app.hello();
    app.init();
    Ok(())
}
