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

### math-stuff

The **math-stuff** program is a set of two basic functionalities: *square* and *sum*
