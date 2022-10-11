import { readInput } from "../utils/index"
const input = readInput().trim().split("\n\n").map( (g) => g.trim().split("\n").map( (r) => r.trim().split("").reduce( (acc:Set<string>, q) => acc.add(q), new Set() ) ) );
const setUnion = function(a, b) {
    for (let value of b) a.add(value);
    return a;
}
const setIntersection = function(a, b) {
    if (a == null) return b;
    for (let value of a) if (!b.has(value)) a.delete(value);
    return a;
}
console.log("Part 1:", input.map( (group) => group.reduce( (acc, person) => setUnion(acc, person), new Set<string>() )).map( (a) => a.size ).reduce( (acc, c) => acc + c, 0));
console.log("Part 2:", input.map( (group) => group.reduce( (acc, person) => setIntersection(acc, person), null )).map( (a) => a.size ).reduce( (acc, c) => acc + c, 0));