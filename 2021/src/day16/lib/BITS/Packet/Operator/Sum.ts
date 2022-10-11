import PacketOperator = require('../Operator')

export class PacketOperatorSum extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 0);
    }
    value(): number { return this.packets.reduce( (acc,p) => acc + p.value(), 0); }

}
