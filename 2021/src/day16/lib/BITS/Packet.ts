import { BitBuffer } from "./BitBuffer";
export interface Packet {
    version:number;
    type:number;
    parse(from:BitBuffer):void;
    versionSum():number;
    value():number;
}

