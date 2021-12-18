import PacketOperator = require('../Operator')

export class PacketOperatorMinimum extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 2);
    }
    value(): number { return this.packets.reduce( (acc,p) => Math.min(acc, p.value()), this.packets[0].value()); }

}
