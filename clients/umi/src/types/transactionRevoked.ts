/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';

export type TransactionRevoked = {
  multisig: PublicKey;
  transaction: PublicKey;
  proposer: PublicKey;
};

export type TransactionRevokedArgs = TransactionRevoked;

export function getTransactionRevokedSerializer(): Serializer<
  TransactionRevokedArgs,
  TransactionRevoked
> {
  return struct<TransactionRevoked>(
    [
      ['multisig', publicKeySerializer()],
      ['transaction', publicKeySerializer()],
      ['proposer', publicKeySerializer()],
    ],
    { description: 'TransactionRevoked' }
  ) as Serializer<TransactionRevokedArgs, TransactionRevoked>;
}
