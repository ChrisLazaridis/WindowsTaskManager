use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::{Error, ErrorKind};
use std::mem::size_of;
use windows::{
    core::*,
    Win32::System::Diagnostics::ToolHelp::*,
    Win32::Foundation::*,
};

mod process;
mod process_tree;

use crate::process::Process;
use crate::process_tree::ProcessTree;

fn get_all_processes() -> Result<Vec<Process>> {
    let mut all_processes = HashMap::new();
    let mut child_parent_pairs: HashMap<i32, Vec<i32>> = HashMap::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).map_err(|e| {
            Error::new(ErrorKind::Other, format!("Failed to create snapshot: {:?}", e))
        })?;

        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
        process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut process_entry).is_ok() {
            loop {
                let pid = process_entry.th32ProcessID as i32;
                let parent_pid = process_entry.th32ParentProcessID as i32;

                let name = CStr::from_ptr(process_entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .into_owned();

                if pid != 0 {
                    let process = Process::new(pid, name);
                    all_processes.insert(pid, process);
                    child_parent_pairs
                        .entry(parent_pid)
                        .or_default()
                        .push(pid);
                }

                if Process32Next(snapshot, &mut process_entry).is_err() {
                    if GetLastError() == ERROR_NO_MORE_FILES {
                        break;
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Failed to retrieve next process").into());
                    }
                }
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to retrieve first process").into());
        }

        CloseHandle(snapshot).ok();
    }

    // Adding children to their respective parent processes
    for (parent_pid, children) in child_parent_pairs {
        if let Some(parent) = all_processes.get_mut(&parent_pid) {
            for child_pid in children {
                parent.add_child(child_pid);
            }
        }
    }

    // Return Ok(all_processes as a vector of processes)
    let processes = all_processes
        .into_iter()
        .map(|(_, process)| process)
        .collect::<Vec<Process>>();
    Ok(processes)
}

fn create_tree(processes: Vec<Process>) -> ProcessTree {
    let mut process_map = HashMap::new();
    let mut process_tree = ProcessTree::new(Process::new(0, "System Idle Process".to_string()));

    // Insert all processes into the map by PID
    for process in processes {
        process_map.insert(process.get_pid(), process);
    }

    // Iterate through the processes again to build the tree
    for process in process_map.values() {
        // Check if the current process is a child process of ANY other process in the map
        // If it is, then wait for the parent process to be added to the tree
        let mut is_child = false;
        for parent_process in process_map.values() {
            if parent_process.has_children() {
                for child_pid in parent_process.get_children() {
                    if *child_pid == process.get_pid() {
                        is_child = true;
                        break;
                    }
                }
            }
            if is_child {
                break;
            }
        }

        // If the process is not a child of any other process in the map, add it to the tree
        if !is_child && !process_tree.exists(process.get_pid()) {
            process_tree.add_child(process_tree.get_root().clone(), process.clone()).unwrap();
        }

        // If the process has children, add them to the tree
        if process.has_children() {
            for child_pid in process.get_children() {
                if let Some(child_process) = process_map.get(child_pid) {
                    if process_tree.add_child(process.clone(), child_process.clone()).is_err() {
                        process_tree.add_child(process_tree.get_root().clone(), child_process.clone()).unwrap();
                    }
                }
                // If the child process is not found in the map, retrieve it from the system
                else if let Ok(child_process) = find_process_by_pid(*child_pid) {
                    process_tree.add_child(process.clone(), child_process).unwrap();
                } else {
                    let child_process = Process::new(*child_pid, "Unknown".to_string());
                    process_tree.add_child(process.clone(), child_process).unwrap();
                }
            }
        }
    }

    process_tree
}


fn find_process_by_pid(pid: i32) -> Result<Process> {
    let mut p = Process::new(pid, "Unknown".to_string());
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).map_err(|e| {
            Error::new(ErrorKind::Other, format!("Failed to create snapshot: {:?}", e))
        })?;

        let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
        process_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut process_entry).is_ok() {
            loop {
                if process_entry.th32ProcessID == pid as u32 {
                    p.set_name(CStr::from_ptr(process_entry.szExeFile.as_ptr())
                        .to_string_lossy()
                        .into_owned());
                    break;
                }

                if Process32Next(snapshot, &mut process_entry).is_err() {
                    if GetLastError() == ERROR_NO_MORE_FILES {
                        break;
                    } else {
                        return Err(Error::new(ErrorKind::Other, "Failed to retrieve next process").into());
                    }
                }
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to retrieve first process").into());
        }

        CloseHandle(snapshot).ok();
    }

    Ok(p)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern "C" fn get_process_tree() -> *mut c_char {
    match get_all_processes() {
        Ok(processes) => {
            let process_tree = create_tree(processes);
            let json_string = match serde_json::to_string(&process_tree) {
                Ok(json) => json,
                Err(_) => String::new(),
            };

            let result = CString::new(json_string).unwrap();
            result.into_raw()
        }
        Err(_) => CString::new("").unwrap().into_raw(),
    }
}
#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { let _ = CString::from_raw(s); }
}