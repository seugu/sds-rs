use crate::header::HistoryEntry;
use crate::record::SDSRecord;

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    MessageReady {
        message_id: String,
        sender_id: String,
        content: Vec<u8>,
    },
    MessageAcknowledged {
        message_id: String,
    },
    MissingDependencies {
        message_id: String,
        missing: Vec<HistoryEntry>,
    },
    /// A sync names messages that are neither delivered nor buffered here. A sync is never
    /// buffered, so this is the only report of what it shows missing.
    SyncNamesMissing {
        sync_id: String,
        missing_dependencies: Vec<HistoryEntry>,
    },
    SendFailed {
        message_id: String,
    },
    Lost {
        message_id: String,
    },
    Resend(SDSRecord),
    Sync(SDSRecord),
    Repair(SDSRecord),
}
