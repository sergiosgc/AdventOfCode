import PacketOperator = require('../Operator')

export class PacketOperatorEqual extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 7);
    }
    value(): number { return this.packets[0].value() == this.packets[1].value() ? 1 : 0; }
}
