"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorMaximum = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorMaximum extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 3);
    }
    value() { return this.packets.reduce((acc, p) => Math.max(acc, p.value()), this.packets[0].value()); }
}
exports.PacketOperatorMaximum = PacketOperatorMaximum;
