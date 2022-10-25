use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::data::UserData;

static USER_DATA_FILENAME: &str = "user_data.yml";

pub fn try_load_user_data(data_dir: &Path) -> Result<Option<UserData>> {
    let data_file = data_dir.join(USER_DATA_FILENAME);

    if !data_file.exists() {
        return Ok(None);
    }

    let user_data = fs::read_to_string(&data_file).context("Failed to read user data file")?;

    let user_data =
        serde_yaml::from_str::<UserData>(&user_data).context("Failed to parse user data")?;

    Ok(Some(user_data))
}

pub fn save_user_data(data_dir: &Path, user_data: &UserData) -> Result<()> {
    if !data_dir.exists() {
        fs::create_dir(&data_dir).context("Failed to create data directory")?;
    }

    let user_data =
        serde_yaml::to_string(&user_data).context("Failed to stringify user data before saving")?;

    fs::write(data_dir.join(USER_DATA_FILENAME), user_data).context("Failed to save user data")?;

    Ok(())
}
