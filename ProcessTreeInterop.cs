using System;
using System.Runtime.InteropServices;


namespace WinTaskManager;

public class ProcessTreeInterop
{
    [DllImport("C:\\Users\\claza\\source\\repos\\WindowsTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr get_process_tree();
    [DllImport("C:\\Users\\claza\\source\\repos\\WindowsTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_c_string(IntPtr str);
    [DllImport("C:\\Users\\claza\\source\\repos\\WindowsTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr get_process_info(uint pid);
    [DllImport("C:\\Users\\claza\\source\\repos\\WindowsTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern bool kill_process_by_pid(uint pid);

    public static string GetProcessTree()
    {
        IntPtr ptr = get_process_tree();
        if (ptr == IntPtr.Zero)
        {
            return null;
        }
        string result = Marshal.PtrToStringAnsi(ptr);
        free_c_string(ptr);
        return result;
    }

    public static string GetProcessInfo(uint pid)
    {
        IntPtr ptr = get_process_info(pid);
        if (ptr == IntPtr.Zero)
        {
            return null;
        }
        string result = Marshal.PtrToStringAnsi(ptr);
        free_c_string(ptr);
        return result;
    }
    public static bool KillProcessByPid(uint pid)
    {
        return kill_process_by_pid(pid);
    }
}