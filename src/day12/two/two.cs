using System;
using System.Collections.Generic;
using System.Linq;
Console.WriteLine(Two.depthFirst(Two.input(), new List<string>( new string[] { "start" })).Count);
class Two {
    public static Boolean canBeVisited(string destination, List<string> soFar) {
        if (destination == "start") return false;
        if (!soFar.Contains(destination)) return true;
        if (destination != destination.ToLower()) return true;

        List<string> smallCaves = new List<string>( soFar.Where( n => n.ToLower() == n ) );
        Dictionary<string, Boolean> test = new Dictionary<string, bool>();
        foreach (string n in smallCaves) test.TryAdd(n, true);
        return smallCaves.Count == test.Count;
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
