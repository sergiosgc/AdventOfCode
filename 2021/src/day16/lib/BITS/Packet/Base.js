"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BasePacket = void 0;
const PacketLiteralValue = require("./LiteralValue");
const PacketOperatorSum = require("./Operator/Sum");
const PacketOperatorProduct = require("./Operator/Product");
const PacketOperatorMinimum = require("./Operator/Minimum");
const PacketOperatorMaximum = require("./Operator/Maximum");
const PacketOperatorGreaterThan = require("./Operator/GreaterThan");
const PacketOperatorLessThan = require("./Operator/LessThan");
const PacketOperatorEqual = require("./Operator/Equal");
class BasePacket {
    constructor(version, type) {
        this.version = version;
        this.type = type;
    }
    static create(from) {
        let result;
        let version = from.getFixedInt(3);
        let type = from.getFixedInt(3);
        switch (type) {
            case 0:
                result = new PacketOperatorSum.PacketOperatorSum(version);
                break;
            case 1:
                result = new PacketOperatorProduct.PacketOperatorProduct(version);
                break;
            case 2:
                result = new PacketOperatorMinimum.PacketOperatorMinimum(version);
                break;
            case 3:
                result = new PacketOperatorMaximum.PacketOperatorMaximum(version);
                break;
            case 4:
                result = new PacketLiteralValue.PacketLiteralValue(version);
                break;
            case 5:
                result = new PacketOperatorGreaterThan.PacketOperatorGreaterThan(version);
                break;
            case 6:
                result = new PacketOperatorLessThan.PacketOperatorLessThan(version);
                break;
            case 7:
                result = new PacketOperatorEqual.PacketOperatorEqual(version);
                break;
            default:
                throw new Error("Unknown operator type " + type);
                break;
        }
        result.parse(from);
        return result;
    }
}
exports.BasePacket = BasePacket;
