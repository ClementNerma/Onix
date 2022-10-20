use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use futures::executor::block_on;
use log::{debug, error, trace};

use crate::data::UserData;

use super::state::WrappedState;

pub struct WritableUserData<'a> {
    inner: &'a mut UserData,
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
        &self.inner
    }
}

impl<'a> DerefMut for WritableUserData<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> Drop for WritableUserData<'a> {
    fn drop(&mut self) {
        error!("YOH BITCH");
        *self.saving_state = UserDataSavingState::Modified;
    }
}

pub type UserDataSaver = Box<dyn Fn(&UserData) -> Result<()> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserDataSavingState {
    Unchanged,
    Modified,
    WaitingForSave,
}

static LOGGER_TARGET: &str = "state-saver";

pub fn user_data_saver(state: WrappedState) -> ! {
    let state = || block_on(state.lock());
    let saving_state = || state().user_data_saving_state;
    let wait = || std::thread::sleep(Duration::from_secs(1));

    loop {
        match saving_state() {
            UserDataSavingState::Unchanged => {
                wait();
                continue;
            }

            UserDataSavingState::Modified => {
                trace!(
                    target: LOGGER_TARGET,
                    "State was modified, waiting for no modification until a delay..."
                );

                while saving_state() == UserDataSavingState::Modified {
                    trace!(target: LOGGER_TARGET, "{:?}", saving_state());

                    state().user_data_saving_state = UserDataSavingState::WaitingForSave;
                    wait();
                }

                trace!(
                    target: LOGGER_TARGET,
                    "| No modification since the defined period of time, saving..."
                );

                let mut state = state();

                if let Err(err) = (state.user_data_saver)(state.user_data()) {
                    error!(target: LOGGER_TARGET, "Failed to save user data: {err:?}");
                } else {
                    debug!(target: LOGGER_TARGET, "| State was successfully saved.");
                }

                state.user_data_saving_state = UserDataSavingState::Unchanged;

                wait();
            }

            UserDataSavingState::WaitingForSave => {
                panic!("Assertion error: user data is unexpectedly waiting for save")
            }
        }
    }
}
