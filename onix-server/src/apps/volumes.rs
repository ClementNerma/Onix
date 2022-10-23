use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

use crate::graphql_enum;

#[derive(SimpleObject, InputObject, Serialize, Deserialize, Clone)]
#[graphql(input_name_suffix = "Input")]
pub struct AppVolume {
    pub name: String,
    pub variant: AppVolumeType,
}

graphql_enum!(
    #[derive(Serialize, Deserialize)]
    pub enum AppVolumeType {
        /// Volume that could be dropped without any real datal loss
        /// (e.g. cache or unimportant configuration files)
        Disposable,

        /// Internal volume used to store data which does not need to be modifiable
        /// by the end user (non-disposable)
        Internal,

        /// External volume stored in an accessible filesystem
        External {
            container_path: String,
            readonly: bool,
        },

        /// Binding to a real directory
        BindToPath {
            real_path: String,
            container_path: String,
            readonly: bool,
        },
        // TODO: /// Binding to a global path
        // #[derive(PartialEq, Eq, Hash)]
        // GlobalPath {
        //     global_path_id: String,
        //     readonly: bool,
        // },
    }
);
