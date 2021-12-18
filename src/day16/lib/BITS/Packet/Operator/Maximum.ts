import PacketOperator = require('../Operator')

export class PacketOperatorMaximum extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 3);
    }
    value(): number { return this.packets.reduce( (acc,p) => Math.max(acc, p.value()), this.packets[0].value()); }

}
