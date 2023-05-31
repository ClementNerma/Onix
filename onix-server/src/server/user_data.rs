use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use log::{debug, error, trace};
use tokio::time::sleep;

use crate::data::UserData;

use super::state::WrappedState;

/// Writable user data, acting as a mechanism to ensure proper data saving.
///
/// While it allows both non-mutable and mutable access to its inner data, it also takes care of the saving.
///
/// It is only generated when a writable access to the user data is requested.
/// When this type is dorpped, it signals that modification happened to the creator through a reference.
/// This allows to ensure the user data are in a consistent state before actually saving them.
pub struct WritableUserData<'a> {
    /// The writable user data
    pub inner: &'a mut UserData,

    /// The (internally) modifiable saving state
    saving_state: &'a mut UserDataSavingState,
}

impl<'a> WritableUserData<'a> {
    pub(super) fn new(inner: &'a mut UserData, saving_state: &'a mut UserDataSavingState) -> Self {
        Self {
            inner,
            saving_state,
        }
    }
}

impl<'a> Deref for WritableUserData<'a> {
    type Target = UserData;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a> DerefMut for WritableUserData<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

/// Notify the parent that user data were modified when this is dropped
impl<'a> Drop for WritableUserData<'a> {
    fn drop(&mut self) {
        *self.saving_state = UserDataSavingState::Modified;
    }
}

pub type UserDataSaver = Box<dyn Fn(&UserData) -> Result<()> + Send + Sync>;

/// State of the user data saving
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserDataSavingState {
    /// No modification happened since the last save
    Unchanged,

    /// The data was modified since the last save
    Modified,

    /// The data was modified since the last save, but the saver waits a bit before performing the actual save
    WaitingForSave,
}

static LOGGER_TARGET: &str = "state-saver";
static SLEEP_DURATION: Duration = Duration::from_secs(1);

/// A loop looking for user data modifications before triggering the actual save
pub async fn user_data_saver(state: WrappedState) -> ! {
    loop {
        match get_saving_state(&state).await {
            UserDataSavingState::Unchanged => {
                sleep(SLEEP_DURATION).await;
                continue;
            }

            UserDataSavingState::Modified => {
                trace!(
                    target: LOGGER_TARGET,
                    "State was modified, waiting for no modification until a delay..."
                );

                while get_saving_state(&state).await == UserDataSavingState::Modified {
                    set_saving_state(&state, UserDataSavingState::WaitingForSave).await;
                    sleep(SLEEP_DURATION).await;
                }

                trace!(
                    target: LOGGER_TARGET,
                    "| No modification since the defined period of time, saving..."
                );

                let mut state = state.lock().await;

                if let Err(err) = (state.user_data_saver)(state.user_data()) {
                    error!(target: LOGGER_TARGET, "Failed to save user data: {err:?}");
                } else {
                    debug!(target: LOGGER_TARGET, "| State was successfully saved.");
                }

                state.user_data_saving_state = UserDataSavingState::Unchanged;

                drop(state);

                sleep(SLEEP_DURATION).await;
            }

            UserDataSavingState::WaitingForSave => {
                panic!("Assertion error: user data is unexpectedly waiting for save")
            }
        }
    }
}

async fn get_saving_state(state: &WrappedState) -> UserDataSavingState {
    state.lock().await.user_data_saving_state
}

async fn set_saving_state(state: &WrappedState, new_value: UserDataSavingState) {
    state.lock().await.user_data_saving_state = new_value;
}
