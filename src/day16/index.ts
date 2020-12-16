import { readFileSync } from "fs"
import { resourceLimits } from "worker_threads";
class FieldRule {
    field: string; min1: number; min2: number; max1: number; max2: number;
    public constructor(init?: Partial<FieldRule>) { 
        if (init) { init.min1 = +init.min1; init.min2 = +init.min2; init.max1 = +init.max1; init.max2 = +init.max2; }
        Object.assign(this, init);
    }
    isValid = value => this.min1 <= value && value <= this.max1 || this.min2 <= value && value <= this.max2
}
const input = ((groups) => {
    return {
        fieldRules: Array.from(groups['fields'].matchAll(/^(?<field>[^:]*): (?<min1>\d+)-(?<max1>\d+) or (?<min2>\d+)-(?<max2>\d+)$/smg)).map( m => new FieldRule(m['groups'])),
        myticket: groups['myticket'].split(",").map( f => +f ),
        tickets: groups['tickets'].trim().split("\n").map( l => l.trim().split(",").map( f => +f ))
    }
})(readFileSync("src/day16/input.txt", "utf-8").match(/(?<fields>.*)^your ticket:[^0-9]*(?<myticket>.*)^nearby tickets:[^0-9]*(?<tickets>.*)/sm)['groups'])

const matrixColumn = (matrix, col) => matrix.map( row => row[col] )
const partB = function(input) {
    let validTickets = input.tickets.filter( ticket => 0 == ticket.filter( fieldValue => !input.fieldRules.map( rule => rule.isValid(fieldValue) ).reduce( (acc, valid) => acc || valid, false) ) )
    let fieldValidityMatrix = input.fieldRules.map( rule => validTickets[0].map( (dummy, fieldIndex) => validTickets.map( ticket => rule.isValid(ticket[fieldIndex] )).reduce( (allValid, valid) => allValid && valid, true )  ))
    let fieldMap = new Map<number,string>()
    for (let found = true; found; ) {
        let newMaps = fieldValidityMatrix[0]
            .map( (dummy, col) => matrixColumn(fieldValidityMatrix, col).map( (valid, index) => valid ? index : null ).filter( valid => valid != null ) ) 
            .map( (validRules, fieldIndex) => validRules.length == 1 ? { rule: validRules[0], field: fieldIndex } : null )
            .filter( v => v != null )
        found = newMaps.length > 0
        for (let m of newMaps) {
            fieldMap.set(m.field, input.fieldRules[m.rule].field)
            fieldValidityMatrix = fieldValidityMatrix.map( (row, index) => row.map( value => value && index != m.rule ) )
        }
    }
    return Array.from(fieldMap.entries()).filter( ([index, fieldName]) => fieldName.startsWith("departure") ).map ( ([index, fieldName]) => input.myticket[index] ).reduce( (acc, v) => acc * v )
}
console.log("Part 1:",input.tickets
    .map( ticket => ticket
        .map( (value) => input.fieldRules
            .map( (rule) => rule.isValid(value) )
            .reduce( (acc, valid) => acc || valid, false )
            ? null : value
        )
        .filter( v => v != null )
    )
    .reduce( (acc, arr) => acc.concat(arr), [])
    .reduce( (acc, v) => acc + v, 0 )
)
console.log("Part 2:",partB(input))