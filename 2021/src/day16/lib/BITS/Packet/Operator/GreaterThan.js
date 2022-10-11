"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorGreaterThan = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorGreaterThan extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 5);
    }
    value() { return this.packets[0].value() > this.packets[1].value() ? 1 : 0; }
}
exports.PacketOperatorGreaterThan = PacketOperatorGreaterThan;
