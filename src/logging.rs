use crate::config::Settings;
use simplelog::*;
use std::sync::Once;
static _INIT_LOGGERS: Once = Once::new();

/// Configure logging backend.
///
/// # Note
/// - This function is safe to call multiple time, the underlining backend will be called once only
pub fn initialize_logging() {
    _INIT_LOGGERS.call_once(|| {
        TermLogger::init(
            Settings::get().logging.into(),
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
        .unwrap();
    });
}
