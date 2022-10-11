import { Packet } from "../Packet";
import { BitBuffer } from "../BitBuffer";
import BasePacket = require("./Base")

export abstract class PacketOperator implements Packet {
    version:number;
    type:number;
    packets:Packet[] = [];
    constructor(version:number, type:number) {
        this.version = version;
        this.type = type;
    }
    parse(from:BitBuffer):void {
        let lengthTypeId:number = from.getBit();
        if (lengthTypeId == 0) {
            let subBufferLen = from.getFixedInt(15);
            let subBuffer:BitBuffer = from.getSubBuffer(subBufferLen);
            while(!subBuffer.isEmpty()) this.packets.push(BasePacket.BasePacket.create(subBuffer));
        } else {
            for (let i = from.getFixedInt(11); i>0; i--) this.packets.push(BasePacket.BasePacket.create(from));
        }
    }
    versionSum(): number {
        let result = this.version;
        for (let i=0; i<this.packets.length; i++) result += this.packets[i].versionSum();
        return result;
    }
    abstract value(): number;
}
