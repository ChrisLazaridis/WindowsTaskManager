using System.Collections.Generic;
using System.Text.Json.Serialization;
namespace WinTaskManager;
public class Process
{
    [JsonPropertyName("pid")]
    public int Pid { get; set; }

    [JsonPropertyName("name")]
    public string Name { get; set; }

    [JsonPropertyName("children_by_id")]
    public List<int> ChildrenById { get; set; } = new List<int>();

    public bool HasChildren() => ChildrenById.Count > 0;

    public void AddChild(int childPid) => ChildrenById.Add(childPid);
}