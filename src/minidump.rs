#![cfg(feature = "minidump")]

use std::fs::File;
use std::path::PathBuf;
use minidump_writer::MinidumpType;
use directories::UserDirs;

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return Err(err.into()),
        }
    }
}

pub fn write_minidump(destination_override: Option<&str>) -> Result<File, anyhow::Error> {
    let destination_dir: PathBuf = match destination_override {
        Some(destination_override) => format!("{destination_override}\\minidump.mdmp").into(),
        None => {
            let user_dir = match UserDirs::new() {
                Some(user_dir) => user_dir,
                None => return Err(anyhow::anyhow!("bruh")),
            };
            let dir = format!("{}\\minidump.mdmp", user_dir.home_dir().display());
            dir.into()
        },
    };
    
    let mut minidump_destination_file: File =
        unwrap_or_return!(File::create(destination_dir));

    // The arguments for the minidump writer function.
    let mdarg_exception_code: Option<i32> = None; // The exception code as an i32.
    let mdarg_thread_id: Option<u32> = Some(thread_id::get() as u32); // The thread id which will be minidumped.
    let mdarg_minidump_type: Option<MinidumpType> = Some(MinidumpType::empty()); // The minidump type (Look at the implementation for all the valid values).
    let mdarg_destination_file: &mut File = &mut minidump_destination_file; // The destination file where the minidump will be written to.

    // Writes the minidump.
    let write_result = minidump_writer::minidump_writer::MinidumpWriter::dump_local_context(
        mdarg_exception_code,
        mdarg_thread_id,
        mdarg_minidump_type,
        mdarg_destination_file,
    );

    match write_result {
        Ok(()) => Ok(minidump_destination_file),
        Err(err) => Err(err.into()),
    }
}