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
  getStructDecoder,
  getStructEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
} from "@solana/kit";

export type AuthEvent = { multisig: Address; programId: Address };

export type AuthEventArgs = AuthEvent;

export function getAuthEventEncoder(): Encoder<AuthEventArgs> {
  return getStructEncoder([
    ["multisig", getAddressEncoder()],
    ["programId", getAddressEncoder()],
  ]);
}

export function getAuthEventDecoder(): Decoder<AuthEvent> {
  return getStructDecoder([
    ["multisig", getAddressDecoder()],
    ["programId", getAddressDecoder()],
  ]);
}

export function getAuthEventCodec(): Codec<AuthEventArgs, AuthEvent> {
  return combineCodec(getAuthEventEncoder(), getAuthEventDecoder());
}
