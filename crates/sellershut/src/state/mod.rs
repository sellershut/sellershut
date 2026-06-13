use bon::Builder;

use crate::logs::LogHandle;

#[derive(Clone, Builder)]
pub struct AppState {
    log_handle: LogHandle,
}
