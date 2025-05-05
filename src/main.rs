use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

// Function types (directly matching the C++ DLL signatures)
type InitializeFn = unsafe extern "C" fn();
type IsAttachedFn = unsafe extern "C" fn() -> bool;
type GetClientsFn = unsafe extern "C" fn() -> *mut ClientInfo;
type ExecuteFn = unsafe extern "C" fn(script: *const u8, clients: *const *const i8, count: i32);

// Struct definition for a single Roblox client
#[repr(C)]
#[derive(Debug)]
struct ClientInfo {
    version: *const i8,
    name: *const i8,
    id: i32,
}

// Helper function: extract a list of client names from GetClients()
fn get_client_names(get_clients: &Symbol<GetClientsFn>) -> Vec<CString> {
    let mut clients = Vec::new();
    unsafe {
        let mut ptr = get_clients();
        while !(*ptr).name.is_null() {
            let name = CStr::from_ptr((*ptr).name).to_string_lossy().into_owned();
            clients.push(CString::new(name).unwrap());
            ptr = ptr.add(1);
        }
    }
    clients
}

fn main() {
    // Load the Nezur.dll library
    let lib = unsafe {
        Library::new("Nezur.dll").expect("❌ Could not load Nezur.dll (is it in the same dir?)")
    };

    unsafe {
        // Load function exports from the DLL
        let initialize: Symbol<InitializeFn> = lib.get(b"Initialize").expect("❌ Missing Initialize()");
        let is_attached: Symbol<IsAttachedFn> = lib.get(b"IsAttached").expect("❌ Missing IsAttached()");
        let get_clients: Symbol<GetClientsFn> = lib.get(b"GetClients").expect("❌ Missing GetClients()");
        let execute: Symbol<ExecuteFn> = lib.get(b"Execute").expect("❌ Missing Execute()");

        println!("🦀 Rust execution wrapper using Nezur API");
        println!("🧠 Nezur CLI (type `help` or `exit`)\n");

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim();

            match command {
                "exit" => break,
                "help" => {
                    println!("Commands:");
                    println!("  attach                     - Attach to Roblox");
                    println!("  execute <lua_code>         - Run a Lua script");
                    println!("  exports                    - Show loaded DLL exports");
                    println!("  exit                       - Quit the CLI");
                }
                "attach" => {
                    println!("🔧 Initializing...");
                    initialize();

                    println!("⏳ Waiting for DLL to attach to Roblox...");
                    sleep(Duration::from_secs(2));

                    println!("🔍 Checking attachment status...");
                    if is_attached() {
                        println!("✅ Successfully attached to Roblox.");
                    } else {
                        println!("❌ Failed to attach DLL.");
                    }
                }
                "exports" => {
                    println!("📦 Exported functions from Nezur.dll:");
                    println!("  - Initialize()");
                    println!("  - IsAttached() -> bool");
                    println!("  - GetClients() -> *ClientInfo");
                    println!("  - Execute(script: *const u8, clients: *const *const i8, count: i32)");
                }
                c if c.starts_with("execute ") => {
                    if !is_attached() {
                        println!("❌ Not attached. Use `attach` first.");
                        continue;
                    }

                    // Smart parser: remove surrounding single or double quotes
                    let lua_code = &c["execute ".len()..].trim();
                    let clean_code = lua_code
                        .trim_start_matches('"')
                        .trim_start_matches('\'')
                        .trim_end_matches('"')
                        .trim_end_matches('\'');

                    println!("📜 Executing script:\n{}", clean_code);

                    let script_bytes = clean_code.as_bytes();

                    // Gather list of currently attached client names
                    let clients_cstring = get_client_names(&get_clients);
                    let client_ptrs: Vec<*const i8> = clients_cstring.iter().map(|s| s.as_ptr()).collect();
                    let client_count = client_ptrs.len() as i32;

                    if client_count == 0 {
                        println!("⚠️ No attached clients found.");
                        continue;
                    }

                    println!("🧠 Executing script on {} client(s)...", client_count);
                    execute(script_bytes.as_ptr(), client_ptrs.as_ptr(), client_count);
                    println!("✅ Script sent.");
                }
                _ => println!("❓ Unknown command. Type `help`."),
            }
        }
    }
}
