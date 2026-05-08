use std::io::Write;

// #[allow(dead_code)]
// pub fn get_image_name_url() -> anyhow::Result<Vec<String>> {
//     let url = url::Url::parse("file:///home/user/.config")?;
//     if url.scheme() != "file" {
//         return Err(anyhow::anyhow!("Error url is not file"));
//     }
//
//     //get file form url
//     let path = url
//         .to_file_path()
//         .map_err(|_| anyhow::anyhow!("Error file url"))?;
//     let mut file_names = Vec::new();
//
//     for entry in std::fs::read_dir(&path)? {
//         let file_name = entry?
//             .file_name()
//             .into_string()
//             .unwrap_or_else(|_| "invalid_name".to_string());
//         file_names.push(file_name);
//     }
//
//     Ok(file_names)
// }

#[allow(dead_code)]
pub fn image_str_vec(path: &str) -> anyhow::Result<Vec<String>> {
    let home = std::env!("HOME");
    let path = format!("{}/{}", home, path);
    std::env::set_current_dir(&path)?;

    let mut image_vec: Vec<String> = Vec::new();
    let read_dir_image = std::fs::read_dir(path)?;
    for pfile in read_dir_image {
        let filename = pfile?
            .file_name()
            .into_string()
            .unwrap_or_else(|_| "None".to_string());
        image_vec.push(filename);
    }
    Ok(image_vec)
}

#[allow(dead_code)]
pub fn read_file() -> anyhow::Result<String> {
    // let home = std::env!("HOME");
    // let path = format!("{home}/{path}");
    // std::env::set_current_dir(home)?;
    //
    // let path = format!("{path}/{file}");
    // let contents = std::fs::read_to_string(file)?;

    let home = std::env!("HOME");
    let path = format!("{home}/.config/bgrunner/imagePick");
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

#[allow(dead_code)]
pub fn overrive_file(path: &str, file: &str, new_ctx: &str) -> anyhow::Result<()> {
    let home = std::env!("HOME");
    let path = format!("{home}/{path}/{file}");
    std::env::set_current_dir(&path)?;

    let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
    file.write_all(new_ctx.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
pub fn sh_overrive_file(image_name: &str) -> anyhow::Result<()> {
    let home = std::env!("HOME");
    let path = format!("{}/.config/bgrunner", home);
    std::env::set_current_dir(&path)?;
    // cmd_lib::run_cmd!("ls")?;
    cmd_lib::run_cmd!("./bgChosse" "$image_name")?;
    // cmd_lib::run_cmd!("$home/.config/bgrunner/bgChosse wow")?;
    Ok(())
}

pub fn sh_reload_feh() -> anyhow::Result<()> {
    let home = std::env!("HOME");
    let path = format!("{}/.config/bgrunner", home);
    std::env::set_current_dir(&path)?;
    // cmd_lib::run_cmd!(pwd)?;
    cmd_lib::run_fun!(sh "./runner.sh")?;
    // cmd_lib::run_cmd!("i3-msg" "restart")?;
    Ok(())
}
