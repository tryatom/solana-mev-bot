# 🤖 Solana MEV Bot (Jito-Enabled)

<div align="center">
  <img src="https://img.shields.io/badge/Solana-14F195?style=for-the-badge&logo=solana&logoColor=black" alt="Solana" />
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Jito-blue?style=for-the-badge&logo=polkadot&logoColor=white" alt="Jito" />
</div>

<br>

An advanced Maximal Extractable Value (MEV) bot purpose-built for the Solana blockchain. By integrating directly with the Jito Block Engine, this bot executes sophisticated atomic arbitrage and sandwich attacks with zero risk of failed transaction reversions.

> **Warning:** Running this software requires significant capital, a deep understanding of Solana's account model, and a high-performance infrastructure setup.

## 🎯 Supported Strategies

Currently, the bot implements the following automated strategies:

- **Atomic Arbitrage:** Monitors price discrepancies of the same asset across Raydium, Orca, and Meteora. Automatically executes cyclical trades (e.g., SOL -> USDC -> SOL) when profitable accounting for fees.
- **Sandwich Attacks:** Scans the mempool for large, pending AMM swaps with high slippage tolerance. The bot bundles transactions to buy before the target and sell immediately after for a guaranteed spread profit.
- **Liquidations:** Continuously calculates health factors for lending protocol positions (e.g., Solend, Marginfi) and executes atomic liquidations the moment they become profitable.
- **Jito Bundle Optimization:** Automatically dynamically calculates the optimal tip amount to send to Jito validators to ensure bundle inclusion without overpaying.

## 🚀 Quick Start

### 1. Prerequisites

- A **Dedicated Solana Node** (do not use public RPCs) connected over a LAN or ultra-low latency link.
- **Jito Block Engine** access (API Key and authorized Keypair).
- A funded wallet containing SOL for transaction fees, Jito tips, and the base capital for arbitrage.
- **Rust** (nightly toolchain recommended for performance features).

### 2. Installation

Clone the repository and build the optimized binary:

```bash
git clone https://github.com/tryatom/solana-mev-bot.git
cd solana-mev-bot
cargo build --release --locked
```

### 3. Configuration

Rename `config.example.toml` to `config.toml` and configure your strategy parameters:

```toml
[network]
rpc_url = "http://127.0.0.1:8899"
ws_url = "ws://127.0.0.1:8900"
jito_block_engine = "frankfurt.mainnet.block-engine.jito.wtf"

[wallet]
keypair_path = "~/.config/solana/id.json"
jito_auth_keypair_path = "~/.config/solana/jito-auth.json"

[strategies.arbitrage]
enabled = true
min_profit_lamports = 100000  # Minimum profit to execute
max_capital_allocation = 10.0 # Max SOL to use per trade

[strategies.sandwich]
enabled = false # Requires deep liquidity and advanced setup
target_programs = ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"] # Raydium CP-Swap
```

### 4. Running the Bot

Run the release binary. The bot will immediately begin profiling the network state.

```bash
./target/release/solana-mev-bot run --config config.toml
```

## 🏗 System Architecture

The MEV bot is designed around a multi-threaded, lock-free architecture:

*   **State Manager (Thread 1):** Ingests raw websocket data (account changes, new blocks) and maintains an in-memory graph of all AMM pool reserves.
*   **Searcher (Threads 2-16):** Uses highly parallelized graph traversal algorithms (Bellman-Ford variants) to identify profitable cycles continuously.
*   **Execution Engine:** Interacts with the Jito SDK to wrap the localized opportunities into signed bundles.

## 📈 Performance Tuning

To maximize PnL, consider the following environmental optimizations:

- **Colocation:** Host your bot on the same AWS/GCP region as the Jito servers (e.g., Frankfurt or Tokyo).
- **Network Tuning:** Increase UDP buffer sizes and disable TCP delay (`TCP_NODELAY`) at the OS level.
- **Custom AMM Curves:** We've provided an interface for injecting optimized mathematical closures for calculating AMM invariants (bypassing slow BPF execution).

## 📄 License & Disclaimer

MIT License. See the `LICENSE` file for details. This is experimental software. The maintainers are not responsible for any lost funds or misconfigured strategies. Use at your own risk.
 
<!-- Documentation update 1 for solana-mev-bot -->
