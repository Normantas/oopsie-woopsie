# Oopsie woopsie!
A crate to make debugging unexpected panics easier both on end-user machines.
Provides a panic handler which generates a helpful panic message and saves a backtrace, system information and minidump to a file.
This is intended to be used only on builds that are distributed to other people.

## Features
- Displays the PanicInfo and a backtrace
- Creates a panic log file which contains the panic message, system info, and a backtrace
- Creates a minidump of the current process (Currently quite limited)

## Usage example
```
fn main() {
    // Add #[cfg(not(debug_assertions))] here if you want to only enable these in release builds
    std::panic::set_hook(Box::new(|panicinfo: &std::panic::PanicInfo| {
        oopsie_woopsie::set_panic_handler(panicinfo, &oopsie_woopsie::PanicHandlerConfig{
            file_dir: Some("D:/Projects/oopsie_woopsie_test".into()),
        })
    }));

    panic!("Goodbye, world!");
}
```

### Reminder: This crate only works for panics. It cannot debug crashes. If you need crash debugging, you need to use an out-of-process crash handler such as [Breakpad](https://github.com/google/breakpad) or [Crashpad](https://github.com/chromium/crashpad). There is also [EmbarkStudio's Crash handling utility crates](https://github.com/EmbarkStudios/crash-handling).