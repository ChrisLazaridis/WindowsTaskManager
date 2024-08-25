using System.Runtime.CompilerServices;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace WinTaskManager
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
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
                Header = $"{processTree.Root.Pid} - {processTree.Root.Name}"
            };

            foreach (var child in processTree.Children.Values)
            {
                item.Items.Add(CreateTreeViewItem(child));
            }

            return item;
        }
    }
    
}