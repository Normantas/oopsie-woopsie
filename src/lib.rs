use std::panic::PanicInfo;

mod minidump;
mod panic_msg;

#[derive(Debug, Clone)]
pub struct PanicHandlerConfig {
    pub file_dir: Option<String>,
}

pub fn set_panic_handler(panicinfo: &PanicInfo, config: PanicHandlerConfig) {
    let grad = panic_msg::get_message(panicinfo);
    println!("{}", "bruh");

    #[cfg(feature = "minidump")]
    {
        let minidump_result = minidump::write_minidump(config.file_dir.as_deref());
        match minidump_result {
            Ok(file_path) => println!("Minidump written to {file_path:?}"),
            Err(err) => println!("Error while writing minidump! Err: {err}"),
        }
    };

        println!("bruh!");
    
}