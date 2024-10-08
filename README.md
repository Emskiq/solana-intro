# Solana Intro Programs

Yet another repository, part of my journey to becoming a Rust Auditor/Blockchain Developer.

Here, I have documented and developed several Solana programs while following [these great tutorials](https://www.youtube.com/playlist?list=PLUBKxx7QjtVnU3hkPc8GF1Jh4DE7cf4n1).

## Programs

### Basic Concepts
Let’s start with some *basic rules* of Solana programs (smart contracts):
- In Solana, everything is an account, similar to Solidity and many other blockchains.
- A "smart contract" in Solana is often referred to as a **program**.
- Each account consists of a public and a private key.
- Accounts can either hold the bytecode of your programs (i.e., the raw logic of your smart contract) or store the state variables of a specific program. This differs from Solidity, where the contract itself holds its state.
- Program-derived data accounts (PDA), which store and manage program state, are created by programs themselves.
- Development and testing of Solana programs often take place directly on the devnet, as it's easier and similar to working in a local environment.

References:
- [Official Solana Documentation](https://solana.com/docs/core/accounts) - Well-formed and simply explained documentation regarding the basic concepts of Solana development.
- [Video Tutorials](https://www.youtube.com/playlist?list=PLUBKxx7QjtVnU3hkPc8GF1Jh4DE7cf4n1)

---

### `hello-solana`

This is my first Solana program written in Rust using the Solana Web3.js SDK—a basic "Hello World" example.

#### Key Points:
- Set up and configured the `Cargo.toml` file to use the Solana Rust SDK.
- Used the `entrypoint` macro to declare the main function that executes whenever a transaction or instruction is sent to our program (essentially, the account holding the program).
- Key parameters for the `process_instruction` function:
  - **`program_id`**: The public key of the current program.
  - **`accounts`**: An array of accounts passed in the transaction instructions when the transaction was sent to our program.
  - **`instruction_data`**: Additional data (in bytes) passed in the transaction.
- The simplest or most practical way to invoke our program is by using the JavaScript Web3 library for Solana, hence we also created a TypeScript client.
- The `client` code demonstrates how a transaction is constructed and sent to our program.
- In our client, we defined a `TransactionInstruction`, where we passed the `program_id`, `accounts` (`keys` field in the Solana Web3.js SDK), and `instruction_data` (`data` field in the Solana Web3.js SDK).

References:
- [Official Solana Documentation for Rust Programs](https://solana.com/developers/guides/getstarted/rust-to-solana)
- [Official Solana Documentation for JS Client](https://solana.com/docs/clients/javascript#interacting-with-custom-programs)
- [Solana Web3.js Documentation](https://solana-labs.github.io/solana-web3.js/)

---

### `math-stuff` and `advanced-math-stuff`

#### Key Points:
- The `math-stuff` program provides basic functionalities like **square** and **sum** operations.
- The `advanced-math-stuff` program extends `math-stuff` by adding **subtraction** and **division**. It introduces the concept of using `instruction_data` to determine which operation to perform, all within the same program.
- Both programs have **state**, which is stored in another account (commonly referred to as a [*data account*](https://solana.com/docs/core/accounts#data-account)), separate from the one holding the program bytecode.
- In the [`math.ts`](math-stuff/src/client/math.ts) file, you can see how the *client account* is created and passed into the `keys` field in `TransactionInstruction` — this account serves as our [data account](https://solana.com/docs/core/accounts#data-account).

References:
- [Official Solana Documentation](https://solana.com/docs/core/accounts#data-account) - Regarding the Solana account model.

---

### `transfer-sol`

This is a basic program for transferring SOL between accounts. You can refer to [the reference repo](https://github.com/Coding-and-Crypto/Rust-Solana-Tutorial/tree/master/transfer-sol) for more information on generating example accounts.

#### Key Points:
- Every account must have a **minimum balance** of SOL to exist on the blockchain. You cannot simply send one lamport to an account with a balance of **0 SOL**. You need to meet the minimum required balance so the account can store its data on-chain.
- The `system_program` module in the Rust Solana crate has many built-in programs to handle common operations, such as transferring SOL.
- In this program, multiple accounts are passed into the `TransactionInstruction`. Notably, one of the accounts must be marked as a **signer** to authorize the transaction.

References:
- [Solana Stack Exchange - Transfer SOL Simulation Error](https://solana.stackexchange.com/questions/7793/error-failed-to-send-transaction-transaction-simulation-failed-transaction-re)


