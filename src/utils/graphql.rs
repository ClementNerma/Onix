use anyhow::Error;

pub type Result<T> = std::result::Result<T, String>;

pub fn format_err(err: Error) -> String {
    // TODO: to improve
    format!("{err:?}")
}
