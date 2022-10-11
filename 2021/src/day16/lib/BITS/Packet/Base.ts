import { Packet } from "../Packet";
import { BitBuffer } from "../BitBuffer";
import PacketLiteralValue = require('./LiteralValue')
import PacketOperator = require('./Operator')
import PacketOperatorSum = require('./Operator/Sum')
import PacketOperatorProduct = require('./Operator/Product')
import PacketOperatorMinimum = require('./Operator/Minimum')
import PacketOperatorMaximum = require('./Operator/Maximum')
import PacketOperatorGreaterThan = require('./Operator/GreaterThan')
import PacketOperatorLessThan = require('./Operator/LessThan')
import PacketOperatorEqual = require('./Operator/Equal')

export abstract class BasePacket implements Packet {
    version:number;
    type:number;
    constructor(version:number, type:number) {
        this.version = version;
        this.type = type;
    }
    public static create(from:BitBuffer):Packet {
        let result;
        let version = from.getFixedInt(3);
        let type = from.getFixedInt(3);
        switch(type) {
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
    abstract parse(from:BitBuffer):void;
    abstract versionSum(): number;
    abstract value(): number;
}
