import { Node } from "./Node";
import { PairNode } from "./PairNode";
export class LeafNode implements Node {
    parentNode:Node|null;
    value:number;
    constructor(parentNode: Node|null, value:number) {
        this.parentNode = parentNode;
        this.value = value;
    }
    depth():number {
        if (this.parentNode == null) throw new Error('Parent is null');
        return 1 + this.parentNode.depth();
    }
    toString():string {
        return this.value.toString();
    }
    tree():Node {
        if (this.parentNode == null) throw new Error('Parent is null');
        return this.parentNode.tree();
    }
    inOrder():Node[] { return [ this ]; }
    shouldExplode():Boolean {  return false; }
    shouldSplit():Boolean { return this.value >= 10; }
    explode():void {}
    split():void {
        if (this.parentNode == null) throw new Error('Parent is null');
        let left = new LeafNode(null, Math.floor(this.value / 2));
        let right = new LeafNode(null, Math.ceil(this.value / 2));
        let replacement = new PairNode(this.parentNode, left, right);
        left.parentNode = replacement;
        right.parentNode = replacement;
        this.parentNode.replaceChild(this, replacement);

    }
    replaceChild(oldChild:Node, newChild:Node):void {}
    magnitude():number { return this.value; }
}