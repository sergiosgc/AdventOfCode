"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorSum = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorSum extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 0);
    }
    value() { return this.packets.reduce((acc, p) => acc + p.value(), 0); }
}
exports.PacketOperatorSum = PacketOperatorSum;
