using System.Collections.Generic;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace WinTaskManager;
public class ProcessTree
{
    [JsonPropertyName("root")]
    public Process Root { get; set; }

    [JsonPropertyName("children")]
    public Dictionary<int, ProcessTree> Children { get; set; } = new Dictionary<int, ProcessTree>();

    public bool Exists(int pid)
    {
        if (Root.Pid == pid)
            return true;

        foreach (var subtree in Children.Values)
        {
            if (subtree.Exists(pid))
                return true;
        }

        return false;
    }

    public void AddChild(Process parent, Process child)
    {
        if (Root.Pid == parent.Pid)
        {
            Children[child.Pid] = new ProcessTree { Root = child };
        }
        else
        {
            foreach (var subtree in Children.Values)
            {
                if (subtree.Exists(parent.Pid))
                {
                    subtree.AddChild(parent, child);
                    return;
                }
            }
        }
    }
    public static ProcessTree Deserialize(string json)
    {
        return JsonSerializer.Deserialize<ProcessTree>(json);
    }
    public ProcessTree Find(int pid)
    {
        if (Root.Pid == pid)
            return this;

        foreach (var subtree in Children.Values)
        {
            var result = subtree.Find(pid);
            if (result != null)
                return result;
        }

        return null;
    }

    public string Serialize()
    {
        return JsonSerializer.Serialize(this, new JsonSerializerOptions { WriteIndented = true });
    }
}