use std::sync::atomic::{AtomicI64, Ordering};

pub struct AtomicTimestamp{
    timestamp: AtomicI64
}
impl AtomicTimestamp{
    pub fn new(timestamp: Option<i64>) -> Self{
        let timestamp: i64 = if let Some(time) = timestamp{
            time
        }
        else{
            -1
        };
        Self{
            timestamp: AtomicI64::new(timestamp)
        }
    }
    pub fn load(&self, ordering: Ordering) -> Option<i64>{
        let timestamp = self.timestamp.load(ordering);
        if timestamp == -1{
            return None
        }
        Some(timestamp)
    }
    pub fn store(&self, timestamp: Option<i64>, ordering: Ordering) -> (){
        let timestamp = if let Some(interaction) = timestamp{
            interaction
        }
        else{
            -1
        };
        self.timestamp.store(timestamp, ordering)
    }
}