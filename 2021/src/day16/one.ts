import { privateEncrypt } from "crypto";
import * as BITS from "./lib/BITS"

process.stdin.setEncoding('utf-8');
let input = ():Promise<string> => new Promise( (resolve, reject) => 
    process.stdin.on('readable', async () => 
        resolve( (await process.stdin.read() ?? "").split("\n").filter( (s:string) => s != "" )[0] )));

async function main() {
    let h = await input();
    let b = "";
    for (let i=0; i<h.length; i++) b += BITS.BitBuffer.hex2int(h[i]);
    let bb = new BITS.BitBuffer(b);
    let packet = BITS.BasePacket.create(bb);
    console.log(packet.versionSum());
}
main();