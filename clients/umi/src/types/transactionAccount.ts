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
  bool,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';

export type TransactionAccount = {
  pubkey: PublicKey;
  isSigner: boolean;
  isWritable: boolean;
};

export type TransactionAccountArgs = TransactionAccount;

export function getTransactionAccountSerializer(): Serializer<
  TransactionAccountArgs,
  TransactionAccount
> {
  return struct<TransactionAccount>(
    [
      ['pubkey', publicKeySerializer()],
      ['isSigner', bool()],
      ['isWritable', bool()],
    ],
    { description: 'TransactionAccount' }
  ) as Serializer<TransactionAccountArgs, TransactionAccount>;
}
