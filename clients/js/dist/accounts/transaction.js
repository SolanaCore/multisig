"use strict";
/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TRANSACTION_DISCRIMINATOR = void 0;
exports.getTransactionDiscriminatorBytes = getTransactionDiscriminatorBytes;
exports.getTransactionEncoder = getTransactionEncoder;
exports.getTransactionDecoder = getTransactionDecoder;
exports.getTransactionCodec = getTransactionCodec;
exports.decodeTransaction = decodeTransaction;
exports.fetchTransaction = fetchTransaction;
exports.fetchMaybeTransaction = fetchMaybeTransaction;
exports.fetchAllTransaction = fetchAllTransaction;
exports.fetchAllMaybeTransaction = fetchAllMaybeTransaction;
const kit_1 = require("@solana/kit");
const types_1 = require("../types");
exports.TRANSACTION_DISCRIMINATOR = new Uint8Array([
    11, 24, 174, 129, 203, 117, 242, 23,
]);
function getTransactionDiscriminatorBytes() {
    return (0, kit_1.fixEncoderSize)((0, kit_1.getBytesEncoder)(), 8).encode(exports.TRANSACTION_DISCRIMINATOR);
}
function getTransactionEncoder() {
    return (0, kit_1.transformEncoder)((0, kit_1.getStructEncoder)([
        ["discriminator", (0, kit_1.fixEncoderSize)((0, kit_1.getBytesEncoder)(), 8)],
        ["multisig", (0, kit_1.getAddressEncoder)()],
        ["programId", (0, kit_1.getAddressEncoder)()],
        ["accounts", (0, kit_1.getArrayEncoder)((0, types_1.getTransactionAccountEncoder)())],
        ["data", (0, kit_1.addEncoderSizePrefix)((0, kit_1.getBytesEncoder)(), (0, kit_1.getU32Encoder)())],
        ["signers", (0, kit_1.getArrayEncoder)((0, kit_1.getBooleanEncoder)())],
        ["didExecute", (0, kit_1.getBooleanEncoder)()],
        ["owner", (0, kit_1.getAddressEncoder)()],
    ]), (value) => (Object.assign(Object.assign({}, value), { discriminator: exports.TRANSACTION_DISCRIMINATOR })));
}
function getTransactionDecoder() {
    return (0, kit_1.getStructDecoder)([
        ["discriminator", (0, kit_1.fixDecoderSize)((0, kit_1.getBytesDecoder)(), 8)],
        ["multisig", (0, kit_1.getAddressDecoder)()],
        ["programId", (0, kit_1.getAddressDecoder)()],
        ["accounts", (0, kit_1.getArrayDecoder)((0, types_1.getTransactionAccountDecoder)())],
        ["data", (0, kit_1.addDecoderSizePrefix)((0, kit_1.getBytesDecoder)(), (0, kit_1.getU32Decoder)())],
        ["signers", (0, kit_1.getArrayDecoder)((0, kit_1.getBooleanDecoder)())],
        ["didExecute", (0, kit_1.getBooleanDecoder)()],
        ["owner", (0, kit_1.getAddressDecoder)()],
    ]);
}
function getTransactionCodec() {
    return (0, kit_1.combineCodec)(getTransactionEncoder(), getTransactionDecoder());
}
function decodeTransaction(encodedAccount) {
    return (0, kit_1.decodeAccount)(encodedAccount, getTransactionDecoder());
}
function fetchTransaction(rpc, address, config) {
    return __awaiter(this, void 0, void 0, function* () {
        const maybeAccount = yield fetchMaybeTransaction(rpc, address, config);
        (0, kit_1.assertAccountExists)(maybeAccount);
        return maybeAccount;
    });
}
function fetchMaybeTransaction(rpc, address, config) {
    return __awaiter(this, void 0, void 0, function* () {
        const maybeAccount = yield (0, kit_1.fetchEncodedAccount)(rpc, address, config);
        return decodeTransaction(maybeAccount);
    });
}
function fetchAllTransaction(rpc, addresses, config) {
    return __awaiter(this, void 0, void 0, function* () {
        const maybeAccounts = yield fetchAllMaybeTransaction(rpc, addresses, config);
        (0, kit_1.assertAccountsExist)(maybeAccounts);
        return maybeAccounts;
    });
}
function fetchAllMaybeTransaction(rpc, addresses, config) {
    return __awaiter(this, void 0, void 0, function* () {
        const maybeAccounts = yield (0, kit_1.fetchEncodedAccounts)(rpc, addresses, config);
        return maybeAccounts.map((maybeAccount) => decodeTransaction(maybeAccount));
    });
}
