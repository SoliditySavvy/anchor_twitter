# Solana Twitter-Like Application

## Project Overview

Welcome to the Solana Twitter-Like Application project repository. This project is an on-chain application that allows users to create tweets, add/remove reactions, and add/remove comments. This README provides an overview of the project, its requirements, and instructions for setup and testing.

## Task Overview

This project involves the development of an on-chain program for Solana. Here's how the program works:

1. **Creating Tweets**: Users can create tweets with a topic and content. The topic can be up to 32 bytes in size, and the content can be up to 500 bytes. The topic serves as a seed for a Program Derived Address (PDA), allowing users to create multiple tweets.
2. **Adding/Removing Reactions**: Users can add or remove reactions (e.g., like/dislike) to a tweet. The program creates a new reaction account with a PDA to store important data. The PDA seeds are chosen to prevent more than one reaction per user on one tweet.
3. **Adding/Removing Comments**: Users can add or remove comments to a tweet. Comments have various fields, including content, which is limited to 500 bytes. The content field is also used as input for the PDA of the comment account.

## Submission Process

The source code of the on-chain program is stored within the `programs/twitter` folder. Please make changes only within this folder to facilitate the evaluation process. The requirements for this task are to pass 100% of the provided tests.

## Deadline

The deadline for this task is Tuesday, October 17th, at 23:59 UTC.

## Project Setup

To get started with this project, you will need the following tools and dependencies:

- [Rust](https://www.rust-lang.org/tools/install) (Stable version is recommended)
- [Solana](https://docs.solana.com/cli/install-solana-cli-tools) (Use v1.17.1)
- [Anchor](https://www.anchor-lang.com/docs/installation) (Use v0.28.0)

### Commands

Once you have the necessary setup, you can use the following commands:

1. Install project dependencies using Yarn (which is installed during Anchor installation):

```bash
yarn install
```

2. Build the project:

```bash
anchor build
```

3. Test the project:

```bash
	anchor test
```

If you encounter any issues or have questions during the installation process, or need further assistance, feel free to initiate a discussion on Discord within the Issues Forum.

## Hints and Useful Links

- [Program Derived Address (PDA)](https://solanacookbook.com/core-concepts/pdas.html)
- [Account Context](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)
- [Account Model](https://solana.wiki/zh-cn/docs/account-model/)
- [Solana Development Course](https://www.soldev.app/course)

## Learn More

Thank you for your interest in our project. If you have any questions or need further information, feel free to reach out to us on [Discord](https://discord.gg/x7qXXnGCsa) or follow us on [Twitter](https://twitter.com/ackeeblockchain?lang=en) for updates. For the latest [Solana News](https://solana.com/news), visit [Solana's Twitter](https://twitter.com/solana).

We appreciate your involvement in the Solana Twitter-Like Application project. Good luck with your development tasks!
