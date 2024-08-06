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
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to retrieve first process").into());
        }

        CloseHandle(snapshot).ok();
    }

    println!("All processes: {}", all_processes.len());

    let mut pending_pairs: Vec<(i32, i32, Process)> = Vec::new();
    for (&pid, &(parent_pid, ref process)) in &all_processes {
        pending_pairs.push((parent_pid, pid, process.clone()));
    }

    let mut processed: HashMap<i32, Process> = HashMap::new();

    let mut progress_made = 0;
    while !pending_pairs.is_empty() {
        let mut has_progress = false;
        pending_pairs.retain(|(parent_pid, pid, process)| {
            if processed.contains_key(pid) {
                println!("Process {} already processed.", pid);
                return false;
            }

            if *parent_pid == 0 {
                let p = process.clone();
                root_process.add_child(p);
                processed.insert(*pid, process.clone());
                println!("Added process {} to root.", pid);
                has_progress = true;
                return false;
            }

            if let Some(parent_process) = processed.get_mut(parent_pid) {
                let p = process.clone();
                parent_process.add_child(p);
                processed.insert(*pid, process.clone());
                println!("Added process {} to parent {}.", pid, parent_pid);
                has_progress = true;
                return false;
            }

            true
        });

        if !has_progress {
            println!("No progress made, breaking out of the loop.");
            break;
        }
    }

    // Additional handling for any remaining unprocessed pairs
    for (parent_pid, pid, process) in pending_pairs {
        if processed.contains_key(&pid) {
            println!("Skipping already processed process {}", pid);
            continue;
        }
        let p = process.clone();
        if parent_pid == 0 {
            root_process.add_child(process);
            processed.insert(pid, p);
        } else {
            if let Some(parent_process) = processed.get_mut(&parent_pid) {
                parent_process.add_child(process);
                processed.insert(pid, p);
                println!("Added process {} to parent {}.", pid, parent_pid);
            } else {
                println!("Orphan process detected: PID {} with Parent PID {}. Attaching to root.", pid, parent_pid);
                root_process.add_child(process);
                processed.insert(pid, p);
            }
        }
    }
    println!("Total processes in tree: {}", processed.len());
    println!("Direct children of root: {}", root_process.get_children().len());

    let processes = root_process.traverse();
    println!("Total processes after traversal: {}", processes.len());

    Ok(root_process)
}

fn main() {
    match get_all_processes() {
        Ok(root_process) => {
            let processes = root_process.traverse();

            for process in processes {
                if process.get_pid() == 4 {
                    if process.has_children() {
                        println!("PID: {}, Name: {}", process.get_pid(), process.get_name());
                    }else{
                        println!("FALSE")
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
