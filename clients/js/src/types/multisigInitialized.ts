/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
} from "@solana/kit";

export type MultisigInitialized = {
  multisig: Address;
  owners: Array<Address>;
  threshold: bigint;
  bump: number;
};

export type MultisigInitializedArgs = {
  multisig: Address;
  owners: Array<Address>;
  threshold: number | bigint;
  bump: number;
};

export function getMultisigInitializedEncoder(): Encoder<MultisigInitializedArgs> {
  return getStructEncoder([
    ["multisig", getAddressEncoder()],
    ["owners", getArrayEncoder(getAddressEncoder())],
    ["threshold", getU64Encoder()],
    ["bump", getU8Encoder()],
  ]);
}

export function getMultisigInitializedDecoder(): Decoder<MultisigInitialized> {
  return getStructDecoder([
    ["multisig", getAddressDecoder()],
    ["owners", getArrayDecoder(getAddressDecoder())],
    ["threshold", getU64Decoder()],
    ["bump", getU8Decoder()],
  ]);
}

export function getMultisigInitializedCodec(): Codec<
  MultisigInitializedArgs,
  MultisigInitialized
> {
  return combineCodec(
    getMultisigInitializedEncoder(),
    getMultisigInitializedDecoder(),
  );
}
