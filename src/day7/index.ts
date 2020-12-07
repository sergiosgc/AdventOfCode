import { readInput } from "../utils/index"
class DirectedGraph {
    directEdges:Map<string, Map<string,number>> = new Map<string, Map<string,number>>();
    reverseEdges:Map<string, Map<string,number>> = new Map<string, Map<string,number>>();

    public constructor(init?: Partial<DirectedGraph>) { Object.assign(this, init); }
    addEdge(from:string, to:string, weight: number):void {
        if (!this.directEdges.has(from)) this.directEdges.set(from, new Map<string,number>());
        if (!this.reverseEdges.has(to)) this.reverseEdges.set(to, new Map<string,number>());
        this.directEdges.get(from).set(to,weight);
        this.reverseEdges.get(to).set(from,weight);
    }
    neighbours = (node:string, reverse:boolean = false):Iterable<string> => (reverse ? this.reverseEdges : this.directEdges).get(node)?.keys() ?? []
    reversed = ():DirectedGraph => new DirectedGraph({ directEdges: this.reverseEdges, reverseEdges: this.reverseEdges })
    floodFill(from:string) {
        let result = new Set<string>();
        let stack = [ from ];
        while (stack.length) {
            let neigh = new Set<string>(Array.from(this.neighbours(stack.pop())).filter( (n) => !result.has(n)));
            for (let value of neigh) {
                result.add(value);
                stack.push(value);
            }
        }
        return result;
    }
}
const input = readInput().trim().split("\n")
    .map( (l) => l['match'](/^(?<color>\w+ \w+) bags contain(?<contained>.*)\.$/)['groups'] )
    .map( (m) => { return {
        color: m.color, 
        contained: m.contained.split(',').map( (edge) => edge.match(/^ ?(?:(?:no other)|(?:(?<count>\d+)) (?<color>\w+ \w+)) bags?/) ).
                filter( (edge) => edge['groups'].color ).map( (edge) => ({
                    from: m.color,
                    to: edge['groups'].color,
                    count: +edge['groups'].count
                }) )
    }})
    .reduce( (graph, node) => {
        node.contained.reduce( (graph, edge) => {
            graph.addEdge( edge.from, edge.to, edge.count);
            return graph;
        }, graph)
        return graph;

    }, new DirectedGraph());
const containedBags = (graph:DirectedGraph, by:string):number => 1 + Array.from(graph.neighbours(by)).map( (bag) => input.directEdges.get(by).get(bag) * containedBags(graph, bag) ).reduce( (acc, n) => acc + n, 0 )
console.log("Part 1:", input.reversed().floodFill("shiny gold").size);
console.log("Part 2:", containedBags(input, "shiny gold")-1);