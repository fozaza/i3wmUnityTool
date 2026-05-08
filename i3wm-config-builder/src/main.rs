use std::{
    env,
    fs::{self},
    path::Path,
};

use cmd_lib::{run_cmd, run_fun};

fn main() -> anyhow::Result<()> {
    // set up
    // run_cmd!(mkdir -p "~/.config/i3/module" "~/.config/i3/old")?;
    //run_fun!("bash mkdir /home/suomi/.config/i3/a")?;
    set_work_flow()?;
    set_up()?;
    check_old_config()?;
    builder_config()?;
    // let paths = fs::read_dir(i3cm)?;
    // for path in paths {
    //     let a = path?;
    //     let b = a.path().to_string_lossy().into_owned();
    //     let contents = fs::read_to_string(&b);
    //     println!("dir {}", b);
    //     println!("contents {}", contents.unwrap());
    // }
    Ok(())
}

fn set_work_flow() -> anyhow::Result<()> {
    let get_home = env::var("HOME")?;
    let get_i3 = format!("{}/.config/i3", get_home);
    let to_dir = Path::new(&get_i3);
    env::set_current_dir(to_dir)?;
    Ok(())
}

fn set_up() -> anyhow::Result<()> {
    //run_cmd!(mkdir -p module old)?;
    if run_fun!(ls | grep module).is_err() {
        run_cmd!(mkdir module)?;
        println!("Create directory module");
    }
    if run_cmd!(ls | grep old).is_err() {
        run_cmd!(mkdir old)?;
        println!("Create directory old");
    }
    Ok(())
}

fn check_old_config() -> anyhow::Result<()> {
    if run_cmd!(ls old | grep config.old).is_ok() {
        run_cmd!(rm -f old/config.old)?;
    }
    if run_cmd!(ls | grep config).is_ok() {
        run_cmd!(mv config old/config.old)?;
    }
    Ok(())
}

fn builder_config() -> anyhow::Result<()> {
    let paths = fs::read_dir("module")?;
    for path in paths {
        let path_str = path?.path().to_string_lossy().into_owned();
        if path_str.find(".conf") == None {
            println!("Fonud file this {} not dot config", path_str);
            continue;
        }
        println!("Read file >>> {}", path_str);
        run_cmd!(
        cat $path_str >> config
        )?;
    }
    Ok(())
}
