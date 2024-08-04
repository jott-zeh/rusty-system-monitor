use crate::system::{get_system, update_cpu_usage};
use sysinfo::Components;

pub fn get_cpu_info() -> (Vec<f32>, f32) {
    (get_cpu_usage(), get_cpu_temperature())
}

fn get_cpu_usage() -> Vec<f32> {
    update_cpu_usage();
    let system = get_system();

    let cpu_usage: Vec<f32> = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    cpu_usage
}

fn get_cpu_temperature() -> f32 {
    let components = Components::new_with_refreshed_list();

    let component = components
        .iter()
        .find(|c| c.label() == "coretemp Package id 0");

    match component {
        Some(component) => component.temperature(),
        None => 0.0,
    }
}
