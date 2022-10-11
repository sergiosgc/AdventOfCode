"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const BITS = require("./lib/BITS");
process.stdin.setEncoding('utf-8');
let coalesce = (v, onnull) => v == null ? onnull : v;
let input = () => new Promise((resolve, reject) => process.stdin.on('readable', () => __awaiter(void 0, void 0, void 0, function* () { var _a; return resolve(((_a = yield process.stdin.read()) !== null && _a !== void 0 ? _a : "").split("\n").filter((s) => s != "")[0]); })));
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        let h = yield input();
        let b = "";
        for (let i = 0; i < h.length; i++)
            b += BITS.BitBuffer.hex2int(h[i]);
        let bb = new BITS.BitBuffer(b);
        let packet = BITS.BasePacket.create(bb);
        console.log(packet.value());
    });
}
main();
