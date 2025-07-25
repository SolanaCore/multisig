/**
 * This code was AUTOGENERATED using the codama library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun codama to update it.
 *
 * @see https://github.com/codama-idl/codama
 */

import {
  Account,
  Context,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  bool,
  bytes,
  mapSerializer,
  publicKey as publicKeySerializer,
  struct,
  u32,
} from '@metaplex-foundation/umi/serializers';
import {
  TransactionAccount,
  TransactionAccountArgs,
  getTransactionAccountSerializer,
} from '../types';

export type Transaction = Account<TransactionAccountData>;

export type TransactionAccountData = {
  discriminator: Uint8Array;
  multisig: PublicKey;
  programId: PublicKey;
  accounts: Array<TransactionAccount>;
  data: Uint8Array;
  signers: Array<boolean>;
  didExecute: boolean;
  owner: PublicKey;
};

export type TransactionAccountDataArgs = {
  multisig: PublicKey;
  programId: PublicKey;
  accounts: Array<TransactionAccountArgs>;
  data: Uint8Array;
  signers: Array<boolean>;
  didExecute: boolean;
  owner: PublicKey;
};

export function getTransactionAccountDataSerializer(): Serializer<
  TransactionAccountDataArgs,
  TransactionAccountData
> {
  return mapSerializer<TransactionAccountDataArgs, any, TransactionAccountData>(
    struct<TransactionAccountData>(
      [
        ['discriminator', bytes({ size: 8 })],
        ['multisig', publicKeySerializer()],
        ['programId', publicKeySerializer()],
        ['accounts', array(getTransactionAccountSerializer())],
        ['data', bytes({ size: u32() })],
        ['signers', array(bool())],
        ['didExecute', bool()],
        ['owner', publicKeySerializer()],
      ],
      { description: 'TransactionAccountData' }
    ),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([11, 24, 174, 129, 203, 117, 242, 23]),
    })
  ) as Serializer<TransactionAccountDataArgs, TransactionAccountData>;
}

export function deserializeTransaction(rawAccount: RpcAccount): Transaction {
  return deserializeAccount(rawAccount, getTransactionAccountDataSerializer());
}

export async function fetchTransaction(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Transaction> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  assertAccountExists(maybeAccount, 'Transaction');
  return deserializeTransaction(maybeAccount);
}

export async function safeFetchTransaction(
  context: Pick<Context, 'rpc'>,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions
): Promise<Transaction | null> {
  const maybeAccount = await context.rpc.getAccount(
    toPublicKey(publicKey, false),
    options
  );
  return maybeAccount.exists ? deserializeTransaction(maybeAccount) : null;
}

export async function fetchAllTransaction(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Transaction[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, 'Transaction');
    return deserializeTransaction(maybeAccount);
  });
}

export async function safeFetchAllTransaction(
  context: Pick<Context, 'rpc'>,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions
): Promise<Transaction[]> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeTransaction(maybeAccount as RpcAccount));
}

export function getTransactionGpaBuilder(
  context: Pick<Context, 'rpc' | 'programs'>
) {
  const programId = context.programs.getPublicKey(
    'solanaCoreMultisig',
    'C9h86YyYMpKViSzRpE7XUPrRVRypu5WTSitJ9n8czcZh'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: Uint8Array;
      multisig: PublicKey;
      programId: PublicKey;
      accounts: Array<TransactionAccountArgs>;
      data: Uint8Array;
      signers: Array<boolean>;
      didExecute: boolean;
      owner: PublicKey;
    }>({
      discriminator: [0, bytes({ size: 8 })],
      multisig: [8, publicKeySerializer()],
      programId: [40, publicKeySerializer()],
      accounts: [72, array(getTransactionAccountSerializer())],
      data: [null, bytes({ size: u32() })],
      signers: [null, array(bool())],
      didExecute: [null, bool()],
      owner: [null, publicKeySerializer()],
    })
    .deserializeUsing<Transaction>((account) => deserializeTransaction(account))
    .whereField(
      'discriminator',
      new Uint8Array([11, 24, 174, 129, 203, 117, 242, 23])
    );
}
