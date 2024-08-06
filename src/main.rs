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
                    all_processes.insert(pid, (parent_pid, process));
                }

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

    let mut pending_pairs: Vec<(i32, i32, Process)> = Vec::new();

    for (&pid, &(parent_pid, ref process)) in &all_processes {
        pending_pairs.push((parent_pid, pid, process.clone()));
    }

    let mut processed: HashMap<i32, Process> = HashMap::new();

    while !pending_pairs.is_empty() {
        let mut has_progress = false;

        pending_pairs.retain(|(parent_pid, pid, process)| {
            if processed.contains_key(pid) {
                return false; // Already processed, skip it
            }

            if *parent_pid == 0 {
                root_process.add_child(process);
                processed.insert(*pid, process.clone());
                has_progress = true;
                return false; // Processed as a direct child of root
            }

            if let Some(parent_process) = processed.get_mut(parent_pid) {
                parent_process.add_child(process);
                processed.insert(*pid, process.clone());
                has_progress = true;
                return false; // Processed successfully
            }

            true // Keep in the list for next round
        });

        if !has_progress {
            // No progress was made in this iteration, avoiding infinite loop
            break;
        }
    }

    // Attach unprocessed processes directly to root (could be orphans or roots themselves)
    for (parent_pid, pid, process) in pending_pairs {
        if processed.contains_key(&pid) {
            continue;
        }

        if parent_pid == 0 {
            root_process.add_child(&process);
        } else {
            if let Some(parent_process) = processed.get_mut(&parent_pid) {
                parent_process.add_child(&process);
            } else {
                root_process.add_child(&process); // Attach to root if parent not found
            }
        }

        processed.insert(pid, process);
    }

    Ok(root_process)
}

fn main() {
    match get_all_processes() {
        Ok(root_process) => {
            let processes = root_process.traverse();
            for process in processes {
                println!("PID: {}, Name: {}", process.get_pid(), process.get_name());
                if process.get_pid() == 4 && process.has_children() {
                    for child in process.get_children() {
                        println!("  Child PID: {}, Name: {}", child.get_pid(), child.get_name());
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}


