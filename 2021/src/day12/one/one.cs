using System;
using System.Collections.Generic;
Console.WriteLine(One.depthFirst(One.input(), new List<string>( new string[] { "start" })).Count);
class One {
    public static Boolean canBeVisited(string destination, List<string> soFar) {
        return destination != destination.ToLower() || !soFar.Contains(destination);
    }
    public static List<List<string>> depthFirst(Dictionary<string, List<string>> edges, List<string> soFar) {
        List<List<string>> result = new List<List<string>>();
        string current = soFar[soFar.Count - 1];
        if (current == "end") {
            result.Add(new List<string>(soFar));
            return result;
        }
        if (!edges.ContainsKey(current)) return new List<List<string>>();
        foreach (string destination in edges[current]) {
            if (!canBeVisited(destination, soFar)) continue;
            soFar.Add(destination);
            result.AddRange(depthFirst(edges, soFar));
            soFar.RemoveAt(soFar.Count - 1);
        }
        return result;
    }
    public static Dictionary<string, List<string>> input() {
        string? line;
        Dictionary<string, List<string>> edges = new Dictionary<string, List<string>>();
        while ((line = Console.ReadLine()) != null) {
            string[] parts;
            parts = line.Split("-");
            if (!edges.ContainsKey(parts[0])) edges.Add(parts[0], new List<string>());
            edges[parts[0]].Add(parts[1]);
            if (!edges.ContainsKey(parts[1])) edges.Add(parts[1], new List<string>());
            edges[parts[1]].Add(parts[0]);
        }
        return edges;
    }
}
