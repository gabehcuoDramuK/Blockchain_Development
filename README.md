## Description: Implementing a State Machine-Based Blockchain with Balance transfers

## Overview

In this assignment, you will be implementing a simplified version of a Proof of Work (PoW) blockchain. The main goal of this assignment is to show the understanding you have of the core principles of blockchain technology, including block creation, proof of work, and validation of blocks. Also show some basic balance tracking and implementing transactions between accounts.

You are provided with the following:

1. Main structure of the code with function signatures, data structures, and tests.
2. Partially completed functions where you will fill in the missing logic (TODOs).
3. Test suite to help validate the correctness of your implementation.

## 🚀 Key Requirements:

1. Blocks are added to the chain through a Proof-of-Work process, and each block must be validated based on its contents.
2. The blockchain should support transfers between accounts with basic balance tracking (simple logging).
3. Ensure that the blockchain’s integrity is maintained through proper hashing.

## Project Structure

### 1. Block Structure

A block should contain:

- Index: The position of the block in the chain.
- Timestamp: When the block was created.
- Previous Hash: The hash of the previous block in the chain.
- Data: The contents or payload of the block.
- Nonce: A number used to solve the Proof-of-Work (PoW) problem during block mining.
- Hash: The unique cryptographic hash of the block, calculated based on its contents (index, timestamp, previous_hash, data, nonce).

### 2. Blockchain Structure

The blockchain should:

- Start with a genesis block.
- Store multiple blocks in sequence.
- Manage balances for accounts.
- Add blocks once they are mined after proof of work is executed.

### 3. Proof of Work

The Proof of Work algorithm is a simple computational puzzle that ensures blocks are added with a sufficient level of difficulty. It should:

- Find a proof such that, when hashed with the previous proof, it produces a hash starting with a number of leading zeros specified by the difficulty.

### 4. Balance Tracking and Transfers

The blockchain should:

- Maintain balances for different accounts in a Map.
- Allow transferring balances between accounts only if the sender has sufficient funds.

## Implementation Details

You are required to implement the following functions within the provided code:

1. **Hashing**: Create the SHA-256 hash of the block.
2. **Adding Blocks**: Implement the logic to add a block to the chain based on its validity.
3. **Proof of Work Algorithm**: Implement the computational logic to find a valid proof.
4. **Balance Transfers**: Implement a method to transfer balances between accounts.

### Function Signatures

> The function signatures provided in the template **MUST NOT BE CHANGED**. If they do then our custom evaluation tests will fail and thus the assignment will be marked as failed.

This ensures consistency across all implementations and makes the evaluation process easier.

## Expected Output

The blockchain must be correctly initialized with a valid genesis block as its starting point. Each block must include a valid proof of work, demonstrating that it has been mined correctly and adheres to the computational requirements. Transactions should only succeed if the sender has a sufficient balance to cover the transfer, thereby ensuring the validity of all operations. Finally, the blockchain must maintain its structural integrity by ensuring that each block correctly references the hash of the previous block, preserving the sequential order of the chain.

## Tests

A set of initial tests is provided to help you validate your solution.

You are encouraged to **write your own tests** (use the 'extra_tests.rs' file) to further validate the correctness and robustness of your implementation. This may include testing edge cases, additional scenarios, or corner cases that you think are relevant.

To run the tests:

```bash
$ cargo test
```

## Evaluation

The solution will be evaluated based on the following:

- Correctness: Does the solution behave as described?
- Completeness: Are all functions implemented correctly without changing the function signatures?
- Code Quality: Is the code well-structured, readable, and follows best practices?
- Testing: Have you written comprehensive tests to cover all scenarios?
- Chain Integrity: Does the blockchain maintain a valid chain where each block’s previous_hash matches the hash of the previous block?

## Submission Guidelines

- **Do not change** any of the functions' signatures provided in the files; You are meant to fill the functions (TODOs) with the correct functionality. If you proceed into changing the name of the functions, or the input/output params or the number of those, the correction tests will be failing and thus your submission.
- Ensure that your code compiles without any warnings or errors. A submition that does not compile cannot be considered as successful.
- Add any additional tests that you think are necessary for validating your solution.

### Tips and notes

- Focus on understanding the core concepts of blockchains, such as Proof of Work and hashing.
- Test incrementally: Implement one function at a time, and write tests to verify it before moving on.
- Make sure you check for edge cases and handle errors gracefully where applicable.
- You do not need to import/add any more packages/crates. Everything that you possibly need is already added.

Good luck, and have fun coding! 🚀
