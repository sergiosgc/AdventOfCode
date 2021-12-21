import { LeafNode } from "./LeafNode";
import { Node } from "./Node";
export class PairNode implements Node {
    parentNode:Node|null;
    left:Node;
    right:Node;
    constructor(parentNode: Node|null, left:Node, right:Node) {
        this.parentNode = parentNode;
        this.left = left;
        this.right = right;
    }
    depth():number {
        if (this.parentNode == null) throw new Error('Parent is null');
        return 1 + this.parentNode.depth();
    }
    toString():string {
        return "[" + this.left.toString() + "," + this.right.toString() + "]";
    }
    tree():Node {
        if (this.parentNode == null) throw new Error('Parent is null');
        return this.parentNode.tree();
    }
    inOrder():Node[] { return this.left.inOrder().concat([ this ], this.right.inOrder()); }
    shouldExplode():Boolean { 
        if (this.depth() > 4) throw new Error("depth for " + this.toString() + " is " + this.depth());
        return this.depth() == 4 && this.left instanceof LeafNode && this.right instanceof LeafNode;
    }
    shouldSplit():Boolean { return false; }
    explode():void {
        if (this.parentNode == null) throw new Error('Parent is null');
        if (!(this.left instanceof LeafNode)) throw new Error('Not a pair, cannot explode');
        if (!(this.right instanceof LeafNode)) throw new Error('Not a pair, cannot explode');
        let nodes = this.tree().inOrder();
        let myIndex;
        for (myIndex = 0; nodes[myIndex] != this; myIndex++);
        let before:LeafNode[] = nodes.slice(0,Math.max(0, myIndex-2)).filter( n => n instanceof LeafNode ).map( (n:Node):LeafNode => {
            if (!(n instanceof LeafNode)) throw new Error("This will never happen");
            return n;
        });
        let after:LeafNode[] = nodes.slice(myIndex+2).filter( n => n instanceof LeafNode).map( (n:Node):LeafNode => {
            if (!(n instanceof LeafNode)) throw new Error("This will never happen");
            return n;
        });
        if (before.length) before[before.length - 1].value += this.left.value;
        if (after.length) after[0].value += this.right.value;
        let replacement = new LeafNode(this.parentNode, 0);
        this.parentNode.replaceChild(this, replacement);
    }
    split():void {}
    replaceChild(oldChild:Node, newChild:Node):void {
        if (oldChild != this.left && oldChild != this.right) throw new Error("oldChild not found");
        if (oldChild == this.left) this.left = newChild; else this.right = newChild;
    }
    magnitude():number { return 3 * this.left.magnitude() + 2 * this.right.magnitude(); }
}