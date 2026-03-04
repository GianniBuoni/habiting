use env_logger::{Builder, DEFAULT_WRITE_STYLE_ENV, Env};

pub mod prelude {
    pub use super::init_logger;
}

pub fn init_logger() {
    let env = Env::default()
        .filter("LOG_LEVEL")
        .write_style(DEFAULT_WRITE_STYLE_ENV);

    Builder::from_env(env).init();
}
