"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BitBuffer = void 0;
const BITSError_1 = require("./BITSError");
class BitBuffer {
    constructor(buffer) {
        this.buffer = "";
        this.buffer = buffer;
    }
    isEmpty() { return this.buffer.length == 0; }
    push(data) {
        this.buffer = data + this.buffer;
    }
    static hex2int(hex) {
        let result = {
            "0": "0000",
            "1": "0001",
            "2": "0010",
            "3": "0011",
            "4": "0100",
            "5": "0101",
            "6": "0110",
            "7": "0111",
            "8": "1000",
            "9": "1001",
            "A": "1010",
            "B": "1011",
            "C": "1100",
            "D": "1101",
            "E": "1110",
            "F": "1111"
        }[hex];
        if (typeof (result) == "string")
            return result;
        throw new BITSError_1.BITSError("Invalid hex value: " + hex.charCodeAt(0));
    }
    getBit() {
        if (this.buffer.length == 0)
            throw new BITSError_1.BITSError("Empty buffer");
        let result = this.buffer[0];
        this.buffer = this.buffer.substring(1);
        if (result == "0" || result == "1")
            return result == "0" ? 0 : 1;
        this.push(BitBuffer.hex2int(result));
        return this.getBit();
    }
    getFixedInt(len) {
        let result = 0;
        for (let i = 0; i < len; i++) {
            result *= 2;
            result += this.getBit();
        }
        return result;
    }
    getVariableInt() {
        let result = 0;
        let keepOnKeepingOn;
        do {
            keepOnKeepingOn = this.getBit() == 1;
            for (let i = 0; i < 4; i++) {
                result *= 2;
                result += this.getBit();
            }
        } while (keepOnKeepingOn);
        return result;
    }
    getSubBuffer(len) {
        let content = "";
        for (let i = 0; i < len; i++)
            content += (this.getBit() == 0 ? "0" : "1");
        return new BitBuffer(content);
    }
}
exports.BitBuffer = BitBuffer;
