import { Node } from "./Node";
import { PairNode } from "./PairNode";

export class Tree implements Node {
    topNode:Node|null = null;
    parentNode:null = null;
    constructor(topNode:Node|null) {
        this.topNode = topNode;
    }
    depth(): number { return -1; }
    toString():string { 
        if (this.topNode == null) return "Tree([])";
        return "Tree(" + this.topNode.toString() + ")";
    }
    tree():Node {
        return this;
    }
    inOrder():Node[] {
        if (this.topNode == null) return [];
        return this.topNode.inOrder();
    }
    add(to:Tree) {
        let result = new Tree(null);
        //console.log(this.toString());
        //console.log("+ " + to.toString());
        if (this.topNode == null) throw new Error("Unable to add tree with null topNode");
        if (to.topNode == null) throw new Error("Unable to add tree with null topNode");
        let topNode = new PairNode(result, this.topNode, to.topNode);
        topNode.left.parentNode = topNode;
        topNode.right.parentNode = topNode;
        result.topNode = topNode;
        result.reduce();
        //console.log("= " + result.toString());
        return result;
    }
    reduce() {
        let actionable:Node[];
        while ( (actionable = this.inOrder().filter( (n:Node) => n.shouldExplode() || n.shouldSplit() )).length > 0 ) {
            let toExplode = actionable.filter( (n:Node) => n.shouldExplode() );
            //console.log("R " + this.toString());
            if (toExplode.length) {
                //console.log("RT" + toExplode[0].toString() + (toExplode[0].shouldExplode() ? " explode" : " split"));
                toExplode[0].explode();
            } else {
                //console.log("RT" + actionable[0].toString() + (actionable[0].shouldExplode() ? " explode" : " split"));
                actionable[0].split();
            }
        }
        //console.log("R " + this.toString());
    }
    shouldExplode():Boolean { return false; }
    shouldSplit():Boolean { return false; }
    explode():void {}
    split():void {}
    replaceChild(oldChild:Node, newChild:Node):void {}
    magnitude():number { return this.topNode?.magnitude() ?? 0; }
}