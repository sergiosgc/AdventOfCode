export class BITSError extends Error {
    constructor(message:string) {
        super(message);
        Object.setPrototypeOf(this, BITSError.prototype);
    }
}