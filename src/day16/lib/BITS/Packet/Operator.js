"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperator = void 0;
const BasePacket = require("./Base");
class PacketOperator {
    constructor(version, type) {
        this.packets = [];
        this.version = version;
        this.type = type;
    }
    parse(from) {
        let lengthTypeId = from.getBit();
        if (lengthTypeId == 0) {
            let subBufferLen = from.getFixedInt(15);
            let subBuffer = from.getSubBuffer(subBufferLen);
            while (!subBuffer.isEmpty())
                this.packets.push(BasePacket.BasePacket.create(subBuffer));
        }
        else {
            for (let i = from.getFixedInt(11); i > 0; i--)
                this.packets.push(BasePacket.BasePacket.create(from));
        }
    }
    versionSum() {
        let result = this.version;
        for (let i = 0; i < this.packets.length; i++)
            result += this.packets[i].versionSum();
        return result;
    }
}
exports.PacketOperator = PacketOperator;
