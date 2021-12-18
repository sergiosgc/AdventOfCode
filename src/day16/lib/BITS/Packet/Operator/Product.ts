import PacketOperator = require('../Operator')

export class PacketOperatorProduct extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 1);
    }
    value(): number { return this.packets.reduce( (acc,p) => acc * p.value(), 1); }
}
