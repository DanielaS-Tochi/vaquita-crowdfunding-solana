# 🐄 Vaquita

Vaquita is a decentralized crowdfunding protocol built on Solana using Anchor.

The protocol allows users to create fundraising campaigns, receive transparent on-chain donations, and automatically handle refunds if the funding goal is not met.

## Features

- Create crowdfunding campaigns
- Transparent on-chain donations
- Automatic refunds if the funding goal is not reached
- Creator withdrawal after deadline

## Architecture

### Campaign Account

- creator
- goal
- deadline
- amount_raised
- claimed

### Donation Account

- donor
- campaign
- amount

## Built With

- Solana
- Anchor