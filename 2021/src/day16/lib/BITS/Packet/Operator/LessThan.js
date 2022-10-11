"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorLessThan = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorLessThan extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 6);
    }
    value() { return this.packets[0].value() < this.packets[1].value() ? 1 : 0; }
}
exports.PacketOperatorLessThan = PacketOperatorLessThan;
