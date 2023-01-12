use std::collections::HashSet;

use once_cell::sync::Lazy;

pub static SYSCALL_HINTS: Lazy<HashSet<&str>> = Lazy::new(|| {
    HashSet::from([
        "syscall_handler.call_contract(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.deploy(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.emit_event(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_block_number(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_block_timestamp(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_caller_address(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_contract_address(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_sequencer_address(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.get_tx_signature(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.library_call(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.storage_read(segments=segments, syscall_ptr=ids.syscall_ptr)",
        "syscall_handler.storage_write(segments=segments, syscall_ptr=ids.syscall_ptr)",
    ])
});

pub const NORMALIZE_ADDRESS_SET_IS_250_HINT: &str = "ids.is_250 = 1 if ids.addr < 2**250 else 0";

pub const NORMALIZE_ADDRESS_SET_IS_SMALL_HINT: &str = r#"# Verify the assumptions on the relationship between 2**250, ADDR_BOUND and PRIME.
ADDR_BOUND = ids.ADDR_BOUND % PRIME
assert (2**250 < ADDR_BOUND <= 2**251) and (2 * 2**250 < PRIME) and (
        ADDR_BOUND * 2 > PRIME), \
    'normalize_address() cannot be used with the current constants.'
ids.is_small = 1 if ids.addr < ADDR_BOUND else 0"#;
