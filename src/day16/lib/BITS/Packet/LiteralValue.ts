import { Packet } from "../Packet";
import { BitBuffer } from "../BitBuffer";

export class PacketLiteralValue implements Packet {
    version:number;
    type:number;
    val:number = 0;
    constructor(version:number) {
        this.version = version;
        this.type = 4;
    }
    parse(from:BitBuffer):void {
        this.val = from.getVariableInt();
    }
    versionSum(): number {
        return this.version;
    }
    value(): number {
        return this.val;
    }
}