use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::thread;
use std::time::Duration;

use clap::{Parser, Subcommand};
use libloading::{Library, Symbol};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClientInfo {
    version: *const c_char,
    name: *const c_char,
    id: i32,
}

#[derive(Parser)]
#[command(name = "Nezur CLI")]
#[command(about = "Rust wrapper for Nezur.dll", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Attach DLL
    Attach,
    /// Execute Lua script
    Execute {
        /// Lua script
        script: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let lib = unsafe { Library::new("src/bin/Nezur.dll") }.expect("Failed to load Nezur.dll");

    unsafe {
        let initialize: Symbol<unsafe extern "C" fn()> =
            lib.get(b"Initialize\0").expect("Missing symbol 'Initialize'");
        let is_attached: Symbol<unsafe extern "C" fn()> =
            lib.get(b"IsAttached\0").expect("Missing symbol 'IsAttached'");
        let get_clients: Symbol<unsafe extern "C" fn() -> *mut ClientInfo> =
            lib.get(b"GetClients\0").expect("Missing symbol 'GetClients'");
        let execute: Symbol<
            unsafe extern "C" fn(*const u8, *const *const c_char, i32),
        > = lib.get(b"Execute\0").expect("Missing symbol 'Execute'");

        match &cli.command {
            Commands::Attach => {
                println!("Calling Initialize...");
                initialize();
                thread::sleep(Duration::from_secs(1));
                println!("Calling IsAttached...");
                is_attached();
                println!("DLL Initialized and attached.");
            }

            Commands::Execute { script } => {
                let mut clients = Vec::new();
                let mut ptr = get_clients();

                // Loop over returned clients
                while !(*ptr).name.is_null() {
                    let name_cstr = CStr::from_ptr((*ptr).name);
                    let name_str = name_cstr.to_str().unwrap_or("").trim();

                    if !name_str.is_empty() {
                        clients.push(CString::new(name_str).unwrap());
                    }

                    ptr = ptr.add(1);
                }

                let client_ptrs: Vec<*const c_char> =
                    clients.iter().map(|s| s.as_ptr()).collect();
                /// Not working execution is impossible.
                println!("Executing script on {} clients...", client_ptrs.len());

                let script_bytes = script.as_bytes();

                execute(
                    script_bytes.as_ptr(),
                    client_ptrs.as_ptr(),
                    client_ptrs.len() as i32,
                );

                println!("Script executed.");
            }
        }
    }
}
