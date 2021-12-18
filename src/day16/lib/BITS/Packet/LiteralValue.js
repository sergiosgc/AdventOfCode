"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.PacketLiteralValue = void 0;
class PacketLiteralValue {
    constructor(version) {
        this.val = 0;
        this.version = version;
        this.type = 4;
    }
    parse(from) {
        this.val = from.getVariableInt();
    }
    versionSum() {
        return this.version;
    }
    value() {
        return this.val;
    }
}
exports.PacketLiteralValue = PacketLiteralValue;
