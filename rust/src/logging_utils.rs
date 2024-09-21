use log::*;

pub fn logger_builder() -> env_logger::Builder {
    let mut result = env_logger::builder();
    result
        .filter_module(&cargo_root_logger_name(), LevelFilter::Trace)
        .parse_default_env();
    result
}

fn cargo_root_logger_name() -> String {
    // TODO get just the root name, before first : ?
    env!("CARGO_PKG_NAME").replace("-", "_")
}
