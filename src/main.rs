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

fn get_all_processes() -> Result<Vec<Process>> {
    let mut all_processes = HashMap::new();
    let mut child_parent_pair_pending: HashMap<i32, Vec<i32>> = HashMap::new(); // parent-vector of children

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
                    all_processes.insert(pid, process);
                    child_parent_pair_pending
                        .entry(parent_pid)
                        .or_default()
                        .push(pid);
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
    // return Ok(all_processes as a vector of processes)
    let processes = all_processes
        .into_iter()
        .map(|(_, process)| process)
        .collect::<Vec<Process>>();
    Ok(processes)
}


fn main() {
    match get_all_processes() {
        Ok(processes) => {
            for process in processes {
                println!("PID: {}, Name: {}", process.get_pid(), process.get_name());
            }
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
