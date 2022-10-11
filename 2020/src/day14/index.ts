import { readFileSync } from "fs"
class Computer {
    memory: Map<number, number> = new Map<number, number>();
    andMask: bigint = BigInt(0xFFFFFFFFF);
    orMask: bigint = BigInt(0);
    flippingMask: bigint = BigInt(0);
    get(address: number) { return this.memory.has(address) ? this.memory.get(address) : 0 }
    set(address: number, value: number) { this.memory.set(address, Number(BigInt(value) & this.andMask | this.orMask)); return this; }
}
class Instruction {
    execute(computer:Computer):Computer { return computer; }
    executeB(computer:Computer):Computer { return computer; }
}
class SetMaskInstruction extends Instruction {
    andMask: bigint;
    orMask: bigint;
    public constructor(init?: Partial<SetMaskInstruction>) { super(); Object.assign(this, init); }
    execute(computer:Computer):Computer { computer.andMask = this.andMask; computer.orMask = this.orMask; computer.flippingMask = ~(~this.andMask | this.orMask); return computer; }
    executeB(computer:Computer):Computer { return this.execute(computer); }
}
class SetMemInstruction extends Instruction {
    address: number;
    value: number;
    public constructor(init?: Partial<SetMemInstruction>) { super(); Object.assign(this, init); }
    execute(computer:Computer):Computer { computer.set(this.address, this.value); return computer; }
    *flippedAddresses(mask: bigint, address: bigint, bit: number):Iterable<bigint> {
        if (bit == 36) return yield address;
        if ((BigInt(2**bit) & mask) > 0) {
            for( let result of this.flippedAddresses(mask, BigInt(2**bit) | address, bit+1) ) yield result
            for( let result of this.flippedAddresses(mask, ~BigInt(2**bit) & address, bit+1) ) yield result
        } else for( let result of this.flippedAddresses(mask, address, bit+1) ) yield result
    }
    executeB(computer:Computer):Computer { 
        for (let address of this.flippedAddresses(computer.flippingMask, BigInt(this.address) | computer.orMask, 0)) { computer.memory.set(Number(address), this.value); }
        return computer; 
    }
}
const input = readFileSync("src/day14/input.txt", "utf-8").trim().split("\n")
    .map( l => l.trim() )
        .map( l => l.match(/^(?:mask = (?<mask>[X01]*))|(?:mem\[(?<address>\d+)\] = (?<value>\d+))/)['groups'] )
        .map( g => {
            if (typeof(g['mask']) == 'undefined') {
                return new SetMemInstruction({address: +g['address'], value: +g['value']});
            } else {
                return new SetMaskInstruction({
                    andMask: BigInt(g['mask'].split("").reduce( (acc, bit) => acc * 2 + (bit == "0" ? 0 : 1), 0 )),
                    orMask: BigInt(g['mask'].split("").reduce( (acc, bit) => acc * 2 + (bit == "1" ? 1 : 0), 0 )),
                });
            }
        })
console.log("Part 1:", Array.from(input.reduce( (computer, instruction) => instruction.execute(computer), new Computer() ).memory.values()).reduce( (acc, v) => acc + v))
console.log("Part 2:", Array.from(input.reduce( (computer, instruction) => instruction.executeB(computer), new Computer() ).memory.values()).reduce( (acc, v) => acc + v))
