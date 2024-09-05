using System;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Controls;

namespace WinTaskManager
{
    public partial class MainWindow : Window
    {
        private int _selected_pid;
        public MainWindow()
        {
            InitializeComponent();
        }

        private void LoadProcessTree_Click(object sender, RoutedEventArgs e)
        {
            // Call the Rust interop function to get the process tree
            string processTreeJson = ProcessTreeInterop.GetProcessTree();

            if (string.IsNullOrEmpty(processTreeJson))
            {
                MessageBox.Show("Failed to retrieve process tree.");
                return;
            }

            // Deserialize the JSON into a ProcessTree object
            var processTree = ProcessTree.Deserialize(processTreeJson);

            // Clear existing items in the TreeView
            ProcessTreeView.Items.Clear();

            // Populate the TreeView
            var rootNode = CreateTreeViewItem(processTree);
            ProcessTreeView.Items.Add(rootNode);
        }

        private TreeViewItem CreateTreeViewItem(ProcessTree processTree)
        {
            var item = new TreeViewItem
            {
                Header = $"{processTree.Root.Pid} - {processTree.Root.Name}",
                Tag = processTree.Root.Pid // Store the PID in the Tag property
            };

            foreach (var child in processTree.Children.Values)
            {
                item.Items.Add(CreateTreeViewItem(child));
            }

            return item;
        }

        private void ProcessTreeView_SelectedItemChanged(object sender, RoutedPropertyChangedEventArgs<object> e)
        {
            if (e.NewValue is TreeViewItem selectedItem)
            {
                // Extract the PID from the Tag property
                if (selectedItem.Tag is int pid)
                {
                    //// Get process information from Rust library
                    //string processInfo = ProcessTreeInterop.GetProcessInfo((uint)pid);

                    //if (!string.IsNullOrEmpty(processInfo))
                    //{
                    //    // Show process information in a MessageBox
                    //    MessageBox.Show(processInfo, $"Process Information (PID: {pid})");
                    //}
                    //else
                    //{
                    //    MessageBox.Show("Failed to retrieve process information.");
                    //}
                    _selected_pid = pid;
                    showProcessInfo(pid);

                }
            }
        }

        private void showProcessInfo(int pid)
        {
            string processInfo = ProcessTreeInterop.GetProcessInfo((uint)pid);
            if (string.IsNullOrEmpty(processInfo)) {
                MessageBox.Show("Failed to retrieve process information.");
                return;
            }
            // display the information in the rich text box
            richTextBox.Document.Blocks.Clear();
            richTextBox.AppendText(processInfo);

        }

        private void button_Click(object sender, RoutedEventArgs e)
        {
            if (_selected_pid == 0)
            {
                MessageBox.Show("Please select a process to kill");
                return;
            }
            // find the children of the selected process
            ProcessTree processTree = ProcessTree.Deserialize(ProcessTreeInterop.GetProcessTree());
            ProcessTree selectedProcess = processTree.Find(_selected_pid);
            if (selectedProcess == null)
            {
                MessageBox.Show("Failed to find the selected process");
                return;
            }
            // kill the selected process
            if (ProcessTreeInterop.KillProcessByPid((uint)_selected_pid))
            {
                MessageBox.Show("Process killed successfully");
            }
            else
            {
                MessageBox.Show("Failed to kill the process");
            }

        }
    }
}
