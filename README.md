### SolanaCoreMultisig Contract

## Environment Setup
1. Install `Rust`

   ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default 1.79.0
   ```

2. Install `Solana `

   ```shell
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

   then run `solana-keygen new` to create a keypair at the default location.

3. install `Anchor`

   ```shell
   # Installing using Anchor version manager (avm) 
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   # Install anchor
   avm install 0.31.1
   avm use 0.31.1
   ```

## Quickstart

Clone the repository and enter the source code directory.
```
git clone https://github.com/SolanaCore/multisig
cd multisig/multisig
```

Build
```
anchor build
```
After building, the smart contract files are all located in the target directory.

Deploy
```
anchor deploy
```
Attention, check your configuration and confirm the environment you want to deploy.
