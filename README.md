# Solana Intro Programs/SCs

Yet another repo for my Rust Auditor/Blockchain Developer path.

Here are some of the programs I have developed while following [these great tutorials](https://www.youtube.com/playlist?list=PLUBKxx7QjtVnU3hkPc8GF1Jh4DE7cf4n1).

## Programs

### hello-solana

This program demonstrates how to interact with the Solana blockchain using Rust and the Solana Web3.js SDK. It involves deploying a basic program to the Solana Devnet and making transactions.

Key insights/Problems during:
- **RPC Providers**: I have encountered limitations with rate limits and RPC provider issues, including the inability to request airdrops and hitting rate limits. Switching to the default Devnet RPC (`https://api.devnet.solana.com`) resolved many issues.
- **Error Handling**: I had to implement retry logic and added delays between requests to avoid triggering the "Too Many Requests" error.
- **Key Management**: The program uses both the program's keypair and a newly generated trigger keypair for transactions. Proper funding of accounts is crucial for successful execution of transactions.

### math-stuff and advance-math-stuff

The **math-stuff** program is a set of two basic functionalities: *square* and *sum*

### transfer-sol

Basic logic for transfering SOL between accounts, you can check the [the reference repo](https://github.com/Coding-and-Crypto/Rust-Solana-Tutorial/tree/master/transfer-sol) for more information regarding how you can generate some example accounts.
<br>
Important note: Apparently every account should have minimum amount of SOL in order to exist on blockchain.
This means that you cannot send just 1 lamport to one of you generate accounts (if it has 0 balance), you need to send that minimum amount in order for account to store its respective data on-chain.
Reference: [Solana Stack-exchange question](https://solana.stackexchange.com/questions/7793/error-failed-to-send-transaction-transaction-simulation-failed-transaction-re)

Public keys:
- Dudi: 3G2mByfoQEoR5krBNLkXbbpckMxvkCgu8sXCKd9K7WXE
- Emo: 5ZANefACt3N3NixjjLHXs9KwL7BY7wtcBAkAU5QkBzk8
- Niko: 2jP96sfTqAocHtDWzjYAJUveMRZEHN7Ap4aA5wahT9x6
- Kiro: 4eNP13hsAu2251wmCwmceeiYR4ASbfDPJ6douL8Djkkr
