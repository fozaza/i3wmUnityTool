use std::thread;
use std::time::Duration;

use clap::Parser;
use cmd_lib::run_cmd;
use sysinfo;
use sysinfo::System;

const CONVERT_DATA: f32 = 1024.0 * 1024.0;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CmdCtx {
    #[arg(short, long, default_value_t = 5)]
    time: u64,
}

#[allow(dead_code)]
impl CmdCtx {
    pub fn run() -> anyhow::Result<()> {
        let cmd_ctx = CmdCtx::parse();
        println!("time {}s.", cmd_ctx.time);

        let key = "LOACL_MEM";
        let local_mem = match std::env::var(key) {
            Ok(v) => v,
            Err(_) => return Err(anyhow::anyhow!("Error not found $LOACAL_MEM in config")),
        };

        let mut sys = System::new_all();
        loop {
            sys.refresh_all();
            let used_mem = (sys.used_memory() as f32 / CONVERT_DATA)
                .round()
                .to_string()
                + " Mib";
            let used_swap = (sys.used_swap() as f32 / CONVERT_DATA).round().to_string() + " Mib";

            println!(
                "Read new memory info\n\t- mem >>> {}\n\t- swap >>> {}",
                used_mem, used_swap
            );

            run_cmd!(echo $used_mem > $local_mem/memory)?;
            run_cmd!(echo $used_swap > $local_mem/swap)?;
            thread::sleep(Duration::from_secs(cmd_ctx.time));
        }
    }
}

fn main() -> anyhow::Result<()> {
    // let mut sys = System::new_all();
    CmdCtx::run()?;
    // loop {
    //     sys.refresh_all();
    //     let mem_buffers = sys.used_memory() as f32 / 1_024.0 / 1_024.0;
    //     println!("mem use {}", mem_buffers);
    //     run_cmd!(echo $mem_buffers > "mem_log").unwrap();
    //     thread::sleep(Duration::from_secs(1));
    // }
    //
    Ok(())
}
