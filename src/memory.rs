use crate::system::{get_system, update_memory_usage};

pub fn get_memory_info() -> (u64, u64) {
    update_memory_usage();
    let system = get_system();

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();

    (total_memory, used_memory)
}
