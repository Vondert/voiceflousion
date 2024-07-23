use std::sync::atomic::{AtomicI64, Ordering};

/// Represents an atomic timestamp for managing interaction times.
pub(crate) struct AtomicTimestamp {
    /// The atomic integer representing the timestamp.
    timestamp: AtomicI64,
}

impl AtomicTimestamp {
    /// Creates a new atomic timestamp.
    ///
    /// # Parameters
    ///
    /// * `timestamp` - The optional initial timestamp.
    ///
    /// # Returns
    ///
    /// A new instance of `AtomicTimestamp`.
    pub(crate) fn new(timestamp: Option<i64>) -> Self {
        let timestamp: i64 = if let Some(time) = timestamp {
            time
        } else {
            -1
        };
        Self {
            timestamp: AtomicI64::new(timestamp),
        }
    }

    /// Loads the current timestamp.
    ///
    /// # Parameters
    ///
    /// * `ordering` - The memory ordering for the load operation.
    ///
    /// # Returns
    ///
    /// An `Option<i64>` containing the current timestamp.
    pub(crate) fn load(&self, ordering: Ordering) -> Option<i64> {
        let timestamp = self.timestamp.load(ordering);
        if timestamp == -1 {
            return None;
        }
        Some(timestamp)
    }

    /// Stores a new timestamp.
    ///
    /// # Parameters
    ///
    /// * `timestamp` - The new timestamp to store.
    /// * `ordering` - The memory ordering for the store operation.
    pub(crate) fn store(&self, timestamp: Option<i64>, ordering: Ordering) {
        let timestamp = if let Some(interaction) = timestamp {
            interaction
        } else {
            -1
        };
        self.timestamp.store(timestamp, ordering)
    }
}