import { readFileSync } from "fs"
const input = ( (earliestDeparture:string, busIDs:string) => {
    return {
        earliestDeparture: +earliestDeparture,
        busIDs: busIDs.split(",").map( (id) => id == "x" ? null : +id )
    };
// @ts-expect-error (input is two lines, but the compiler has no way of knowing that)
})(...readFileSync("src/day13/input.txt", "utf-8").trim().split("\n").map( (l) => l.trim() ))
const modularReciprocal = (a: bigint, modulus: bigint) => { for (let candidate = 1n; ; candidate++) if (a * candidate % modulus == 1n) return candidate; }

// Chinese remainder theorem implemented from here: https://www.cut-the-knot.org/blue/chinese.shtml
const chineseRemainderTheorem = (integerEquations:{remainder:bigint, modulus:bigint}[]) => {
    const solutionModulus = integerEquations.reduce( (acc, e) => acc * e.modulus, 1n ) // For a solution to exist, numbers must be co-prime, so no need for an LCM
    return integerEquations
        .map( e => e.remainder * modularReciprocal(solutionModulus / e.modulus, e.modulus) * solutionModulus / e.modulus )
        .reduce( (acc, v) => (acc + v) ) % solutionModulus
}
console.log("Part 1:", ((bus) => bus.id * (bus.departure - input.earliestDeparture))(
    input.busIDs
        .filter( (id) => id )
        .map ( (id) => ({ id: id, departure: id * Math.ceil( input.earliestDeparture / id ) }))
        .reduce( (min, bus) => bus.departure < min.departure ? bus : min ) 
) )
console.log("Part 2:", chineseRemainderTheorem( input.busIDs.map( (bus, index) => ({ remainder: bus == null ? null : BigInt( (bus - index) % bus), modulus: bus == null ? null : BigInt(bus)}) ).filter( eq => eq.modulus != null ) ) )