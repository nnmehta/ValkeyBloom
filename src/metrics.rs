use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use valkey_module::{InfoContext, ValkeyResult};

lazy_static! {
    pub static ref BLOOM_NUM_OBJECTS: AtomicU64 = AtomicU64::new(0);
    pub static ref BLOOM_OBJECT_TOTAL_MEMORY_BYTES: AtomicUsize = AtomicUsize::new(0);
    pub static ref BLOOM_NUM_FILTERS_ACROSS_OBJECTS: AtomicU64 = AtomicU64::new(0);
}

pub fn bloom_info_handler(ctx: &InfoContext) -> ValkeyResult<()> {
    ctx.builder()
        .add_section("bloom_core_metrics")
        .field(
            "bloom_total_memory_bytes",
            BLOOM_OBJECT_TOTAL_MEMORY_BYTES
                .load(Ordering::Relaxed)
                .to_string(),
        )?
        .field(
            "bloom_num_objects",
            BLOOM_NUM_OBJECTS.load(Ordering::Relaxed).to_string(),
        )?
        .field(
            "bloom_num_filters_across_objects",
            BLOOM_NUM_FILTERS_ACROSS_OBJECTS
                .load(Ordering::Relaxed)
                .to_string(),
        )?
        .build_section()?
        .build_info()?;

    Ok(())
}
