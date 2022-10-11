import { notDeepEqual } from "assert";
import { hasUncaughtExceptionCaptureCallback } from "process";
import { LeafNode } from "./lib/LeafNode";
import { Node } from "./lib/Node";
import { PairNode } from "./lib/PairNode";
import { Tree } from "./lib/Tree";
process.stdin.setEncoding('utf-8');

let input = ():Promise<string[]> => new Promise( (resolve, reject) => 
    process.stdin.on('readable', async () => 
        resolve( (await process.stdin.read() ?? "")
            .split("\n")
            .filter( (s:string):boolean => s != "" ) )));

async function main() {
    let tree = (await input()).map( (s:string) => {
        let result = new Tree(null);
        let stack:Node[] = [];
        for (let i=0; i<s.length; i++) {
            switch (s[i]) {
                case '[':
                case ',':
                    break;
                case '0':
                case '1':
                case '2':
                case '3':
                case '4':
                case '5':
                case '6':
                case '7':
                case '8':
                case '9':
                    stack.push( new LeafNode(null, parseInt(s[i])));
                    break;
                case ']':
                    let right = stack.pop();
                    if (typeof(right) == "undefined") throw new Error("Empty stack");
                    let left = stack.pop();
                    if (typeof(left) == "undefined") throw new Error("Empty stack");
                    let pair = new PairNode(null, left, right);
                    left.parentNode = pair;
                    right.parentNode = pair;
                    stack.push(pair);
            }
        }
        if (stack.length != 1) throw new Error("Unbalanced parens");
        stack[0].parentNode = result;
        result.topNode = stack[0];
        return result;
    }).reduce( (a,b) => a.add(b) );
    console.log(tree.magnitude());
}
main();
