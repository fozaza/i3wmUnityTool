use std::{fs, vec};

use serde::{Deserialize, Serialize};
use toml::to_string;

pub fn setup() -> anyhow::Result<()> {
    let home = std::env!("HOME");
    std::env::set_current_dir(home)?;
    Ok(())
}

// vec path file
pub fn get_vpfile() -> anyhow::Result<Vec<String>> {
    let read_dir = std::fs::read_dir(".config/i3/routine")?;
    let mut config_file: Vec<String> = Vec::new();
    for pfile in read_dir {
        let file = pfile?
            .path()
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| "None".to_string());
        config_file.push(file);
    }
    Ok(config_file)
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TypeAlert {
    CRITICAL,
    NORMAL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigRoutine {
    pub hour: u64,
    pub minute: u64,
    pub title: String,
    pub body: String,
    pub type_alert: Option<TypeAlert>,
}

#[derive(Debug)]
pub struct ProcessConfig {
    pub config_routine: ConfigRoutine,
    pub action: bool,
}

pub fn read_config() -> anyhow::Result<Vec<ProcessConfig>> {
    let vpfile = get_vpfile()?;
    let mut vec_config = Vec::new();

    for i in vpfile {
        let context = fs::read_to_string(i)?.trim().to_string();
        let config_routine: ConfigRoutine = match toml::from_str(&context) {
            Ok(v) => v,
            Err(e) => {
                println!("{e:#?}");
                continue;
            }
        };

        let procces_config = ProcessConfig {
            config_routine: config_routine,
            action: false,
        };
        vec_config.push(procces_config);
    }
    Ok(vec_config)
}

#[allow(dead_code)]
pub fn test_out() -> anyhow::Result<()> {
    let data = ConfigRoutine {
        hour: 1,
        minute: 00,
        title: "title".to_string(),
        body: "body".to_string(),
        type_alert: Some(TypeAlert::NORMAL),
    };
    let toml_data = toml::to_string(&data)?;
    std::fs::write("outtest.toml", toml_data)?;
    Ok(())
}
