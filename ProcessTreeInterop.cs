using System;
using System.Runtime.InteropServices;


namespace WinTaskManager;

public class ProcessTreeInterop
{
    [DllImport("C:\\Users\\claza\\source\\repos\\WinTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr get_process_tree();
    [DllImport("C:\\Users\\claza\\source\\repos\\WinTaskManager\\task_manager_lib\\target\\release\\task_manager_lib.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void free_c_string(IntPtr str);

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
}