"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorMinimum = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorMinimum extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 2);
    }
    value() { return this.packets.reduce((acc, p) => Math.min(acc, p.value()), this.packets[0].value()); }
}
exports.PacketOperatorMinimum = PacketOperatorMinimum;
