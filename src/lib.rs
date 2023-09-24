use std::panic::PanicInfo;

mod minidump;
mod panic_msg;

#[derive(Debug, Clone)]
pub struct PanicHandlerConfig {
    pub file_dir: Option<String>,
}

pub fn set_panic_handler(panicinfo: &PanicInfo, _config: &PanicHandlerConfig) {
    let panic_message = panic_msg::get_message(panicinfo);
    println!("{}", &panic_message);

    #[cfg(feature = "minidump")]
    {
        let minidump_result = minidump::write_minidump(_config.file_dir.as_deref());
        match minidump_result {
            Ok(file_path) => println!("Minidump written to {file_path:?}"),
            Err(err) => println!("Error while writing minidump! Err: {err}"),
        }
    };

    #[cfg(feature = "panic_log")]
    {
        let minidump_result = panic_msg::write_panic_log(panic_message, _config.file_dir.as_deref());
        match minidump_result {
            Ok(file_path) => println!("Panic log written to {file_path:?}"),
            Err(err) => println!("Error while writing panic log! Err: {err}"),
        }
    };
}