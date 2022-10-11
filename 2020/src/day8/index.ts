import { readInput } from "../utils/index"
class VM {
    program: Opcode[];
    acc: number = 0;
    pc: number = 0;
    public constructor(init?: Partial<VM>) { Object.assign(this, init); }
    clone() { return new VM({ acc: this.acc, pc: this.pc, program: this.program.map( (op) => op.clone() ) }) } 
    op() { this.program[this.pc].op(this); }
    terminated = () => this.pc >= this.program.length
}
class Opcode {
    opcode: string; argument: number; 

    public constructor(init?: Partial<Opcode>) { Object.assign(this, init); }
    clone() { return new Opcode({opcode: this.opcode, argument: this.argument }) }
    op(vm:VM) { this["op_" + this.opcode].apply(this, [vm]) }
    op_nop(vm:VM) { vm.pc++; }
    op_acc(vm:VM) { vm.acc += this.argument; vm.pc++; }
    op_jmp(vm:VM) { vm.pc += this.argument; }
}
const runUntilLoop = function(vm:VM) {
    for (let visited:Set<number> = new Set<number>(); !visited.has(vm.pc) && !vm.terminated(); vm.op() ) visited.add(vm.pc);
    return vm;
}
const fixedVMs = function*(vm:VM) {
    let fixes = new Map<string, string>([[ "nop", "jmp" ], [ "jmp", "nop" ]])
    for (let i=0; i<vm.program.length; i++) if (fixes.has(vm.program[i].opcode)) {
        let fix = vm.clone()
        fix.program[i].opcode = fixes.get(fix.program[i].opcode);
        yield fix;
    } 
}
const input = new VM({acc: 0, pc:0, program: readInput().trim().split("\n")
    .map( (l) => l.trim().match(/^(?<opcode>\w+) (?<argument>.*)/)['groups'] )
    .map( (g) => new Opcode({opcode: g['opcode'], argument: parseInt(g['argument'])}))})
console.log("Solution to part 1:", runUntilLoop(input.clone()).acc);
console.log("Solution to part 2:", Array.from(fixedVMs(input.clone())).map ( (vm) => runUntilLoop(vm) ).filter( (vm) => vm.terminated() ).map ( (vm) => vm.acc ).pop() )
