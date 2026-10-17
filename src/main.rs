// ============================================================================
//  BOILERPLATE : PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // This line makes the sysinfo tools available. You will read from `sys`.
    let mut sys = System::new_all();
    sys.refresh_all();

    // Section 1 TODO: processors.
    //   Read sys.cpus().len() and print "Logical processors : {}".
    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    // Section 2 TODO: memory.
    //   Read sys.total_memory() and sys.used_memory() (both in bytes), print
    //   total and free in GB (divide by GB), and set mem_load to the percent.
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB)  : {:.2}", total as f64 / GB);
    println!("Memory free  (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%)  : {}", mem_load);

    // Section 3 TODO: uptime.
    //   Print System::uptime() / 60 as "Uptime (minutes) : {}".
    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    // Section 4 TODO: disk.
    //   Make `let disks = Disks::new_with_refreshed_list();` then take the
    //   first disk, read total_space() and available_space(), print them in
    //   GB, and set disk_used_pct to the percent in use.
    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct = 0.0;
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        let mut used_pct = 0.0;
        if dtotal > 0 {
            used_pct = 100.0 * (dtotal - dfree) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)    : {:.2}", dtotal as f64 / GB);
        println!("Disk free  (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used  (%)     : {:.2}", used_pct);
        disk_used_pct = used_pct;
    }

    // Section 5: score (given, once the two variables above are real)
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}
