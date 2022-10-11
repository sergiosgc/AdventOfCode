"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketOperatorProduct = void 0;
const PacketOperator = require("../Operator");
class PacketOperatorProduct extends PacketOperator.PacketOperator {
    constructor(version) {
        super(version, 1);
    }
    value() { return this.packets.reduce((acc, p) => acc * p.value(), 1); }
}
exports.PacketOperatorProduct = PacketOperatorProduct;
