"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.BITSError = void 0;
class BITSError extends Error {
    constructor(message) {
        super(message);
        Object.setPrototypeOf(this, BITSError.prototype);
    }
}
exports.BITSError = BITSError;
