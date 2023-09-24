#![cfg(feature = "panic_log")]

use std::fs::File;
use std::path::PathBuf;
use directories::UserDirs;
use std::panic::PanicInfo;

pub fn get_message(panicinfo: &PanicInfo) -> String {
    let mut panic_location: String = "Could not get panic location.".to_string();
    let panic_payload: String = panic_message::panic_info_message(panicinfo).to_string();

    if let Some(location) = panicinfo.location() {
        panic_location = format!(
            "{}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    }

    format!(
        "\n\nThe current process panicked!\n\nPanic location: {panic_location}\nPanic payload: {panic_payload}\n\n\n",
    )
}

pub fn write_panic_log(panic_msg: String, destination_override: Option<String>) -> Result<PathBuf, anyhow::Error> {
    let destination_dir: PathBuf = match destination_override {
        Some(destination_override) => format!("{destination_override}\\panic_log.txt").into(),
        None => {
            let user_dir = match UserDirs::new() {
                Some(user_dir) => user_dir,
                None => return Err(anyhow::anyhow!("bruh")),
            };
            let dir = format!("{}\\panic_log.txt", user_dir.home_dir().display());
            dir.into()
        },
    };

    let write_result = std::fs::write(destination_dir, panic_msg);

    match write_result {
        Ok(()) => Ok(destination_dir.into()),
        Err(err) => Err(err.into()),
    }
}