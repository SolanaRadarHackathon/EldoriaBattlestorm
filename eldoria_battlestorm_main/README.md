
# Eldoria Battlestorm: Echoes of Eldoria Mobile

Welcome to **Eldoria Battlestorm: Echoes of Eldoria Mobile**, a turn-based strategy game built on the Solana blockchain! In this mobile adaptation, players engage in strategic battles using unique characters to claim victory. The game leverages Solana's fast and secure blockchain for a seamless and fair mobile gaming experience.

## Table of Contents

- [Getting Started](#getting-started)
- [Game Overview](#game-overview)
- [Smart Contracts](#smart-contracts)
- [Building and Running](#building-and-running)
- [Mobile Integration](#mobile-integration)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust and Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Solana CLI**: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- **Node.js and npm**: [Install Node.js](https://nodejs.org/)
- **Unity** (for mobile integration): [Install Unity](https://unity.com/)

### Installation

Clone the repository:

```sh
git clone https://github.com/yourusername/eldoria-battlestorm-mobile.git
cd eldoria-battlestorm-mobile
```

Install the dependencies:

```sh
cargo build-bpf
```

## Game Overview

Eldoria Battlestorm Mobile is a turn-based strategy game where players control a team of characters, each with unique abilities. The objective is to outmaneuver your opponent and deplete their characters' health to win the game.

### Characters

Each character has the following attributes:

- **mint**: Unique identifier for the character.
- **health**: Health points of the character.
- **hit**: Attack power of the character.
- **xp**: Experience points of the character.

### Game State

The game state is managed by the `GameState` struct, which includes:

- **is_initialized**: Indicates if the game is initialized.
- **initializer**: Public key of the player who initialized the game.
- **guest**: Public key of the guest player.
- Character attributes for both initializer and guest.
- **last_play_time**: Timestamp of the last move.
- **whose_turn**: Indicates whose turn it is to play.

### Actions

Players can perform actions such as:

- **Attack**: Engage in combat with the opponent's characters.
- **Move**: Strategically position characters on the battlefield (to be further developed for the mobile interface).

## Smart Contracts

The core game logic is implemented in Rust using Borsh for serialization. The key structs are:

- `RandomNumber`: Represents a random number for game mechanics.
- `GameState`: Manages the state of the game.
- `Character`: Defines the attributes of a character.
- `Attack`: Represents an attack action.
- `PlayerAccount`: Tracks player stats.

### NFT Mechanics

Characters in **Eldoria Battlestorm Mobile** are represented as NFTs, allowing for ownership and trade of in-game assets. When a character is defeated, the NFT may either **burn** or **reduce its durability** as part of the game mechanics.

## Building and Running

Compile the smart contracts:

```sh
cargo build-bpf
```

Deploy the contracts to the Solana blockchain:

```sh
solana program deploy path/to/your_program.so
```

### Frontend (Mobile Version)

For mobile, the game integrates with Unity. To run the frontend:

```sh
npm install
npm start
```

### Mobile Integration

1. **Unity SDK**: The game is built using the Unity SDK to allow cross-platform mobile support. The blockchain integration is handled using the **Magicblock Unity SDK**.
2. **Blockchain Features**: Players' in-game actions such as purchasing characters, engaging in battles, and upgrading abilities are backed by Solana smart contracts.
3. **Token-based Features**: Players can stake game tokens to participate in special battles, unlock features, or upgrade their characters.

## Mobile Features

- **NFT Durability**: Characters lose durability as they participate in battles, adding an additional strategic layer for the player.
- **Multiplayer**: Battle with other players in real-time through secure Solana blockchain-based interactions.
- **Hardcore Battle Mode**: Enter high-stakes battles where losing could burn your NFTs.
- **Play Modes**: Multiple game modes including tournaments and ranked play.

## Contributing

Contributions are welcome! If you'd like to contribute to **Eldoria Battlestorm Mobile**, follow these steps:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -am 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Create a new Pull Request.

## License

Eldoria Battlestorm is licensed under the MIT License. See `LICENSE` for more information.

---

Enjoy playing **Eldoria Battlestorm Mobile**! For any issues or support, please open an issue on GitHub.

