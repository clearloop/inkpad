//! Event interface
use crate::derive::Host;
use ceres_derive::host;
use ceres_executor::{derive::Value, Error, Result};
use ceres_sandbox::Sandbox;
use ceres_std::Vec;

/// Deposit a contract event with the data buffer and optional list of topics. There is a limit
/// on the maximum number of topics specified by `event_topics`.
///
/// - topics_ptr - a pointer to the buffer of topics encoded as `Vec<T::Hash>`. The value of this
///   is ignored if `topics_len` is set to 0. The topics list can't contain duplicates.
/// - topics_len - the length of the topics buffer. Pass 0 if you want to pass an empty vector.
/// - data_ptr - a pointer to a raw data buffer which will saved along the event.
/// - data_len - the length of the data buffer.
#[host(seal0)]
pub fn seal_deposit_event(
    topics_ptr: u32,
    topics_len: u32,
    data_ptr: u32,
    data_len: u32,
) -> Result<Value> {
    fn has_duplicates<T: Ord>(items: &mut Vec<T>) -> bool {
        // # Warning
        //
        // Unstable sorts are non-deterministic across architectures. The usage here is OK
        // because we are rejecting duplicates which removes the non determinism.
        items.sort_unstable();
        // Find any two consecutive equal elements.
        items.windows(2).any(|w| match w {
            [a, b] => a == b,
            _ => false,
        })
    }

    if data_len > sandbox.max_value_size() {
        return Err(Error::TopicValueTooLarge);
    }

    let mut topics: Vec<[u8; 32]> = match topics_len {
        0 => Vec::new(),
        _ => sandbox.read_sandbox_memory_as(topics_ptr, topics_len)?,
    };

    // If there are more than `event_topics`, then trap.
    if topics.len() > sandbox.ext.schedule.limits.event_topics as usize {
        return Err(Error::TooManyTopics);
    }

    // Check for duplicate topics. If there are any, then trap.
    // Complexity O(n * log(n)) and no additional allocations.
    // This also sorts the topics.
    if has_duplicates(&mut topics) {
        return Err(Error::DuplicateTopics);
    }

    let event_data = sandbox.read_sandbox_memory(data_ptr, data_len)?;
    sandbox.deposit_event(topics, event_data);
    Ok(Value::F32(0))
}
