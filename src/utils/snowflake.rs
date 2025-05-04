use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::process;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Snowflake ID structure:
// - 41 bits for timestamp (milliseconds since epoch)
// - 10 bits for machine/node ID
// - 12 bits for sequence number

const EPOCH: i64 = 1735689600000; // 2025-01-01 00:00:00 UTC
const NODE_ID_BITS: i64 = 10;
const SEQUENCE_BITS: i64 = 12;

const MAX_NODE_ID: i64 = (1 << NODE_ID_BITS) - 1;
const MAX_SEQUENCE: i64 = (1 << SEQUENCE_BITS) - 1;

const TIMESTAMP_SHIFT: i64 = NODE_ID_BITS + SEQUENCE_BITS;
const NODE_ID_SHIFT: i64 = SEQUENCE_BITS;

pub struct SnowflakeGenerator {
    node_id: i64,
    last_timestamp: AtomicI64,
    sequence: AtomicI64,
}

impl SnowflakeGenerator {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<Self, &'static str> {
        let node_id = Self::generate_node_id();

        Ok(Self {
            node_id,
            last_timestamp: AtomicI64::new(0),
            sequence: AtomicI64::new(0),
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn generate_node_id() -> i64 {
        let mut hasher = DefaultHasher::new();

        // Use process ID
        let pid = process::id();
        pid.hash(&mut hasher);

        // Use hostname if available
        if let Ok(hostname) = env::var("HOSTNAME") {
            hostname.hash(&mut hasher);
        }

        // Use a random component from current time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        now.hash(&mut hasher);

        // Get the hash and constrain it to our node ID range
        (hasher.finish() % (MAX_NODE_ID as u64 + 1)) as i64
    }

    pub fn generate(&self) -> Result<i64, &'static str> {
        let current_timestamp = Self::timestamp();
        let last_timestamp = self.last_timestamp.load(Ordering::Acquire);

        if current_timestamp < last_timestamp {
            return Err("Clock moved backwards, refusing to generate ID");
        }

        let sequence = if current_timestamp == last_timestamp {
            // Same millisecond, increment sequence
            let seq = (self.sequence.fetch_add(1, Ordering::SeqCst) + 1) & MAX_SEQUENCE;

            if seq == 0 {
                // Sequence exhausted, wait for next millisecond
                let new_timestamp = Self::wait_next_millis(last_timestamp);
                self.last_timestamp.store(new_timestamp, Ordering::Release);
                0 // Reset sequence
            } else {
                seq
            }
        } else {
            // Different millisecond, reset sequence
            self.sequence.store(0, Ordering::SeqCst);
            self.last_timestamp
                .store(current_timestamp, Ordering::Release);
            0
        };

        // Use the appropriate timestamp (either current or the one from wait_next_millis)
        let timestamp_to_use = self.last_timestamp.load(Ordering::Acquire);

        // Compose the ID without using abs()
        // Ensure we only use 63 bits (leaving the sign bit as 0)
        let timestamp_bits = ((timestamp_to_use - EPOCH) & ((1 << 41) - 1)) << TIMESTAMP_SHIFT;
        let node_bits = (self.node_id & ((1 << NODE_ID_BITS) - 1)) << NODE_ID_SHIFT;
        let sequence_bits = sequence & ((1 << SEQUENCE_BITS) - 1);

        let id = timestamp_bits | node_bits | sequence_bits;

        // The ID will always be positive because we:
        // 1. Use only 63 bits total (41 + 10 + 12 = 63)
        // 2. Leave the most significant bit (sign bit) as 0
        Ok(id)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64
    }

    fn wait_next_millis(last_timestamp: i64) -> i64 {
        let mut timestamp = Self::timestamp();
        while timestamp <= last_timestamp {
            timestamp = Self::timestamp();
        }
        timestamp
    }
}

lazy_static! {
    pub static ref SNOWFLAKE_GENERATOR: SnowflakeGenerator = SnowflakeGenerator::new().unwrap();
}
