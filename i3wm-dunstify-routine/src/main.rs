use std::os::unix::process;

pub mod cmd;

fn main() -> anyhow::Result<()> {
    // cmd::test_out()?;

    cmd::setup()?;
    let mut config = cmd::read_config()?;
    if config.is_empty() {
        println!("Config Process is empty Exit Progarm");
        return Ok(());
    }

    let mut local_now: Vec<u64>;
    let mut time_cooldown: u64;
    loop {
        local_now = chrono::Local::now()
            .format("%H:%M")
            .to_string()
            .split(":")
            .map(|part| part.parse().unwrap())
            .collect();
        time_cooldown = if local_now[1] > 30 {
            30 - (local_now[1] >> 1)
        } else {
            30 - local_now[1]
        };
        println!("next process >>> {time_cooldown}");

        for i in 0..config.len() {
            println!("Struct config {:#?}", config[i].config_routine);
            let hour = config[i].config_routine.hour;
            let minute = config[i].config_routine.minute;
            let title = config[i].config_routine.title.to_string();
            let body = config[i].config_routine.body.to_string();
            let type_alert = match config[i].config_routine.type_alert {
                Some(v) => match v {
                    cmd::TypeAlert::CRITICAL => "critical",
                    cmd::TypeAlert::NORMAL => "normal",
                },
                None => "normal",
            };
            let action = config[i].action;

            println!(
                "time{:?}\nif {}",
                local_now,
                hour <= local_now[0] && minute <= local_now[1] //&& action == false
            );
            if hour <= local_now[0] && minute <= local_now[1] && action == false {
                cmd_lib::run_cmd!(dunstify $title $body "-u" $type_alert)?;
                config[i].action = true;
            }
        }

        std::thread::sleep(std::time::Duration::from_mins(time_cooldown));
    }
}
