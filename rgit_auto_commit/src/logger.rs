use log::{info, LevelFilter};
use systemd_journal_logger::{connected_to_journal, JournalLog};

pub(crate) fn init_logger() {
    JournalLog::new().expect("can't init journaled logger even when found").install().expect("Can't connect to journaled");
    // }
    log::set_max_level(LevelFilter::Info);
    info!(target: "_TEST_LOG_TARGET", "connected_to_journal -> {}", connected_to_journal());
}