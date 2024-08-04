use once_cell::sync::Lazy;
use std::sync::Mutex;
use sysinfo::System;

pub static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| {
    let mut sys = System::new_all();
    sys.refresh_all();
    Mutex::new(sys)
});

pub fn get_system() -> std::sync::MutexGuard<'static, System> {
    SYSTEM.lock().unwrap()
}

pub fn update_cpu_usage() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_cpu_usage();
}

pub fn update_memory_usage() {
    let mut sys = SYSTEM.lock().unwrap();
    sys.refresh_memory();
}
