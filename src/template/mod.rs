mod aptos;
mod base;
mod evm;
mod solana;
mod sui;

use anyhow::Result;
use evm::EvmTemplate;
use log::info;
use solana::SolanaTemplate;

use std::process::Stdio;

pub enum AppType {
    Evm,
    Solana,
    Aptos,
    Sui,
    Base,
    None,
}

const BASE_REPO: &str = "https://github.com/rustweb3/rustapp_template.git";

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

    fn project_file(self: &Self, path: &str) -> String {
        format!("{}/{}", self.name(), path)
    }

    fn replace_file_content(&self, file: &str, from: &str, to: &str) {
        let content = std::fs::read_to_string(file).unwrap();
        let new_content = content.replace(from, to);
        std::fs::write(file, new_content).unwrap();
    }

    fn clone_base(self: &Self) {
        info!("Cloning base repo: {}", &BASE_REPO);
        match std::process::Command::new("git")
            .arg("clone")
            .arg(BASE_REPO)
            .arg(self.name())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut child) => {
                let _ = child.wait();
            }
            Err(e) => {
                info!("Error: {}", e);
            }
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
            "base" => AppType::Base,
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
        AppType::Base => Box::new(base::BaseTemplate {
            name: name.to_string(),
            version: "0.1".to_string(),
            init_shell: vec!["base".to_string(), "init".to_string()],
        }),
    };

    app.clone_base();
    app.replace_file_content(
        &app.project_file("Cargo.toml"),
        "rustapp_template",
        &app.name(),
    );
    app.replace_file_content(
        &app.project_file("src/config.rs"),
        "rustapp_template",
        &app.name(),
    );
    app.replace_file_content(
        &app.project_file("src/bin/main.rs"),
        "rustapp_template",
        &app.name(),
    );
    app.init();
    Ok(())
}
