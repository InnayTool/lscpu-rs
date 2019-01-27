
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;


fn main() {
    let v = read_basic_info();
    for e in v {
        println!("{:?}", e);
    }
}
#[derive(Debug)]
struct Cpuinfo {
    n : u32,
    vendor : String,
    cpu_family : u32,
    model : u32,
    model_name : String,
    stepping : u32,
    microcode : String,
    cpu_mhz : f32,
    cache_size: u32,
    id : u32,
    siblings : u32,
    core_id : u32,
    cpu_cores : u32,
    apicid : u32,
    initial_apicid : u32,
    fpu : bool,
    fpu_exception : bool,
    cpuid_level : u32,
    wp : bool,
    /*flags : Vec<String>,
    bugs : Vec<String>,
    bogomips : u32,
    clflush_size : u32,
    cache_alignment : u32,
    address_size : u32,
    power_management : String,*/
}

impl Cpuinfo {
    fn new() -> Self {
        Cpuinfo {
            n : 0,
            vendor : "".to_string(),
            cpu_family : 0,
            model : 0,
            model_name : "".to_string(),
            stepping : 0,
            microcode : "".to_string(),
            cpu_mhz : 0.0,
            cache_size : 0,
            id : 0,
            siblings : 0,
            core_id : 0,
            cpu_cores : 0,
            apicid : 0,
            initial_apicid : 0,
            fpu : false,
            fpu_exception : false,
            cpuid_level : 0,
            wp : false,
        }
    }
}


fn read_basic_info() -> Vec<Cpuinfo> {
    let mut cpus = Vec::new();
    if let Ok(fd) = File::open("/proc/cpuinfo") {
        let f = BufReader::new(fd);
        let mut cpu = Cpuinfo::new();
        for l in f.lines() {
            cpu.vendor = "".to_string();
            let l = l.unwrap();
            if  l.len() == 0 {
                cpus.push(cpu);
                cpu = Cpuinfo::new();
            } else if l.starts_with("processor") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.n = y;
            } else if l.starts_with("vendor_id") {
                let z: Vec<&str> = l.split(": ").collect();
                cpu.vendor = z[1].to_string();
            } else if l.starts_with("cpu family") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.cpu_family = y;
            } else if l.starts_with("model name") {
                let z: Vec<&str> = l.split(": ").collect();
                cpu.model_name = z[1].to_string();
            } else if l.starts_with("model") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.model = y;
            } else if l.starts_with("stepping") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.stepping = y;
            } else if l.starts_with("microcode") {
                let z: Vec<&str> = l.split(": ").collect();
                cpu.microcode = z[1].to_string();
            } else if l.starts_with("cpu MHz") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : f32 = z[1].parse().unwrap();
                cpu.cpu_mhz = y;
            } else if l.starts_with("cache size") {
                let z: Vec<&str> = l.split(": ").collect();
                let x : Vec<&str> = z[1].split(" ").collect();
                let y : u32 = x[0].parse().unwrap();
                cpu.cache_size = y;
            } else if l.starts_with("physical id") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.id = y;
            } else if l.starts_with("siblings") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.siblings = y;
            } else if l.starts_with("core id") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.core_id = y;
            } else if l.starts_with("cpu cores") {
                let z: Vec<&str> = l.split(": ").collect();
                let x : Vec<&str> = z[1].split(" ").collect();
                let y : u32 = x[0].parse().unwrap();
                cpu.cpu_cores = y;
            } else if l.starts_with("apicid") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.apicid = y;
            } else if l.starts_with("initial apicid") {
                let z: Vec<&str> = l.split(": ").collect();
                let y : u32 = z[1].parse().unwrap();
                cpu.initial_apicid = y;
            } else if l.starts_with("fpu_exception") {
                let z: Vec<&str> = l.split(": ").collect();
                if z[1] == "yes" {
                    cpu.fpu_exception = true;
                } else if z[1] == "false" {
                    cpu.fpu_exception = false;
                }
            } else if l.starts_with("fpu") {
                let z: Vec<&str> = l.split(": ").collect();
                if z[1] == "yes" {
                    cpu.fpu = true;
                } else if z[1] == "false" {
                    cpu.fpu = false;
                }
            } else if l.starts_with("cpuid level") {
                let z: Vec<&str> = l.split(": ").collect();
                let x : Vec<&str> = z[1].split(" ").collect();
                let y : u32 = x[0].parse().unwrap();
                cpu.cpuid_level = y;
            } else if l.starts_with("wp") {
                let z: Vec<&str> = l.split(": ").collect();
                if z[1] == "yes" {
                    cpu.wp = true;
                } else if z[1] == "false" {
                    cpu.wp = false;
                }
            }
        }
    }
    cpus

}