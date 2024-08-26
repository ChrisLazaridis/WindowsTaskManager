using System;
using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Controls;

namespace WinTaskManager
{
    public partial class MainWindow : Window
    {
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
                    // Get process information from Rust library
                    string processInfo = ProcessTreeInterop.GetProcessInfo((uint)pid);

                    if (!string.IsNullOrEmpty(processInfo))
                    {
                        // Show process information in a MessageBox
                        MessageBox.Show(processInfo, $"Process Information (PID: {pid})");
                    }
                    else
                    {
                        MessageBox.Show("Failed to retrieve process information.");
                    }
                }
            }
        }
    }
}
