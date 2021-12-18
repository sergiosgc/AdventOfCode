import PacketOperator = require('../Operator')

export class PacketOperatorGreaterThan extends PacketOperator.PacketOperator {
    constructor(version:number) {
        super(version, 5);
    }
    value(): number { return this.packets[0].value() > this.packets[1].value() ? 1 : 0; }

}
