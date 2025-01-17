use {
    crate::{
        cli,
        config::MyConfig,
        hooks::{AppHooks, HookPoint},
        utils,
    },
    clap::Parser,
    std::sync::{Arc, Mutex},
};

pub struct RunTime {
    pub config: MyConfig,
    pub cli: cli::Cli,
    pub hooks: AppHooks,
    pub context: Arc<Mutex<Context>>,
}

pub struct Context {
    pub value: u64,
}

impl Context {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

pub struct InitOptions {
    pub config_merge_env: bool,
    pub config_merge_cli: bool,
}

impl RunTime {
    pub fn init() {
        let app_dir = utils::app_dir("/config").unwrap();
        if !app_dir.exists() {
            std::fs::create_dir_all(&app_dir).unwrap();
        }
        let config_path = utils::default_app_config_path().unwrap();
        if !config_path.exists() {
            let default_config = MyConfig::default();
            let config_content = toml::to_string(&default_config).unwrap();
            std::fs::write(&config_path, config_content).unwrap();
        }
    }
    pub fn new() -> Self {
        let cli: cli::Cli = cli::Cli::parse();
        Self {
            config: MyConfig::from_cli(&cli),
            cli,
            hooks: AppHooks::new(),
            context: Arc::new(Mutex::new(Context::new())),
        }
    }

    pub fn do_init(&mut self, options: InitOptions) {
        utils::log_init(self.config.main.debug || self.cli.debug);
        if options.config_merge_env {
            self.config.merge_env();
        }
        if options.config_merge_cli {
            self.config.merge_cli(&self.cli);
        }
        match self.hooks.run_hooks(HookPoint::RunInit, &self) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Error running hooks: {} ", e);
            }
        }
    }
}
