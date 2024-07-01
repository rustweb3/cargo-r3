mod app;
mod cli;
mod config;
mod hooks;
mod jobs;
mod template;
mod utils;

use {
    anyhow::{Ok, Result},
    clap::Parser,
    hooks::HookPoint,
    log::{debug, error, info},
};

#[tokio::main]
async fn main() -> Result<()> {
    app::RunTime::init();
    let mut runtime = app::RunTime::new();

    runtime.hooks.register_hook(HookPoint::RunInit, |_hook| {
        info!("Rustweb3 will be great!");
        Ok(())
    });

    runtime.do_init(app::InitOptions {
        config_merge_env: true,
        config_merge_cli: true,
    });
    if runtime.cli.name.is_none() && runtime.cli.command.is_none() {
        error!("Please input the command");
        return Err(anyhow::anyhow!("Please input the command"));
    }

    match runtime.cli.command {
        Some(cli::Command::Test { list }) => {
            debug!("this is debug message !");
            info!("list value is :  {}", list);
            jobs::hello_job(&runtime);
        }
        Some(cli::Command::New { name, app_type }) => {
            let app_type = template::AppType::from(app_type.as_str());
            template::new_app(&name, app_type)?;
        }
        _ => {
            error!("Please input the command");
            cli::Cli::parse_from(&["rustapp", "--help"]);
            return Err(anyhow::anyhow!("Please input the command"));
        }
    }

    Ok(())
}
