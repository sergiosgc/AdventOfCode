"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorEqual = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorEqual extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 7);
    }
    value() { return this.packets[0].value() == this.packets[1].value() ? 1 : 0; }
}
exports.PacketOperatorEqual = PacketOperatorEqual;
