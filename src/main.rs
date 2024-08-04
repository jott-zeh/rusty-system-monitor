mod cpu;
mod memory;
mod system;

fn main() {
    let (cpu_utilization, cpu_temperatures) = cpu::get_cpu_info();
    let (total_memory, used_memory) = memory::get_memory_info();

    println!("CPU Usage: {:?}", cpu_utilization);
    println!("CPU Temperatures: {:?}", cpu_temperatures);
    println!(
        "Used {} MB of total {} MB Memory",
        used_memory / 1024 / 1024,
        total_memory / 1024 / 1024
    );
}
