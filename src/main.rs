use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use windows::{
    core::*,
    Win32::System::Diagnostics::ToolHelp::*,
    Win32::Foundation::*,
};

mod process;
use crate::process::Process;


fn get_all_processes() -> Result<Process> {
    let mut all_processes = HashMap::new(); // Stores all processes by their PID
    let mut root_process = Process::new(0, "System Idle Process".to_string());

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).map_err(|e| {
            Error::new(ErrorKind::Other, format!("Failed to create snapshot: {:?}", e))
        })?;

        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
        process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut process_entry).is_ok() {
            loop {
                let pid = process_entry.th32ProcessID as i32;
                let parent_pid = process_entry.th32ParentProcessID as i32;
                let name = CStr::from_ptr(process_entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();

                if pid != 0 {
                    let process = Process::new(pid, name);

                    // Store process in HashMap
                    all_processes.insert(pid, (parent_pid, process));
                }

                // Try to get the next process
                if Process32Next(snapshot, &mut process_entry).is_err() {
                    let error_code = GetLastError();
                    if error_code == ERROR_NO_MORE_FILES {
                        break;
                    } else {
                        return Err(windows::core::Error::from_win32());
                    }
                }
            }
        }

        CloseHandle(snapshot).ok();
    }

    // Vector to store parent-child relationships
    let mut parent_child_pairs = Vec::new();

    // Collect parent-child relationships
    for (&pid, &(parent_pid, ref process)) in &all_processes {
        parent_child_pairs.push((parent_pid, pid, process.clone()));
    }

    // Create the process tree
    for (parent_pid, _pid, process) in parent_child_pairs {
        if parent_pid != 0 {
            if parent_pid == 4 {
                println!("Y");
            }
            if let Some((_, parent_process)) = all_processes.get_mut(&parent_pid) {

                parent_process.add_child(&process);
            } else {
                // If parent process is not found, this might be an orphaned process.
                // Attach it to root_process or handle it as needed.
                root_process.add_child(&process);
            }
        } else {
            // Attach directly to the root process
            root_process.add_child(&process);

        }
    }

    Ok(root_process)
}

fn main() {
    match get_all_processes() {
        Ok(root_process) => {
            let processes = root_process.traverse();
            for process in processes {
                if process.get_pid() == 4 {
                    println!("Y1");
                    if process.has_children() {
                        println!("Y2");
                    }
                }
                println!("PID: {}, Name: {}", process.get_pid(), process.get_name());
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
