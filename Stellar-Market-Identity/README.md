# Stellar Market Identity (SMI) 🏪🆔

**Empowering the Informal Economy with On-Chain Reputation.**

Stellar Market Identity is a protocol designed to build portable, verifiable business identities for the massive informal economy in emerging markets (market traders, transport operators, street vendors). By analyzing transaction flows and behavior on the Stellar network, SMI generates a dynamic reputation score that allows micro-merchants to access credit and financial services without a traditional banking footprint.

## 🌟 Vision
Most Web3 reputation systems focus on DeFi traders or DAO voters. SMI focuses on **real-world economic activity**. It turns a merchant's wallet address into a sovereign business identity, where every transaction builds their creditworthiness.

---

## 🛠️ Tech Stack

### Blockchain (Soroban)
- **Identity Registry:** Smart contract mapping wallet addresses to business profiles (DID-style).
- **Reputation Token:** Non-transferable "Soulbound" tokens (SBT) or dynamic NFT metadata representing score tiers.
- **Event Logging:** Efficient event emission for off-chain indexers.

### AI & Data Infrastructure
- **Ingestion Pipeline:** Custom indexer to track Stellar transaction history for registered merchants.
- **Scoring Engine:** Python/FastAPI service using XGBoost/LightGBM to calculate reliability scores based on transaction regularity, volume, and counterparty diversity.

### Frontend
- **Framework:** React / Next.js (TypeScript).
- **Wallet:** Freighter integration for signing.
- **Styling:** Tailwind CSS with Dark/Light mode support.

---

## 📂 Project Structure

```bash
Stellar-Market-Identity/
├── frontend/             # Merchant Dashboard & Public Explorer
│   ├── GUIDE.md          # 📘 Integration Guide for Frontend Devs
│   └── src/              # Source code (TypeScript)
│
├── smartcontract/        # Soroban Identity Contracts
│   ├── GUIDE.md          # 📘 Smart Contract Development Guide
│   └── contracts/        # Rust source code
│
├── infrastructure/       # (Planned) Data ingestion & AI scoring
│
├── ISSUES.md             # 🚦 Active Task Tracker
├── CONTRIBUTING.md       # Contribution Guidelines
└── CODE_OF_CONDUCT.md    # Community Standards
```

---

## 🚀 Getting Started

### 1. Smart Contracts
We use **Soroban** (Rust). If you are new to Stellar development, please read the **[Smart Contract Guide](./smartcontract/GUIDE.md)** included in this repo.

```bash
cd smartcontract
cargo test
```

### 2. Frontend
The frontend is built with **Vite/Next.js** and **TypeScript**. Read the **[Frontend Integration Guide](./frontend/GUIDE.md)** for details on connecting Freighter.

```bash
cd frontend
npm install
npm run dev
```

---

## 🤝 How to Contribute

1.  **Pick an Issue:** Check `ISSUES.md` for open tasks.
2.  **Follow Guidelines:** Read `CONTRIBUTING.md` and `STYLE.md`.
3.  **Modular Commits:** We strictly enforce **modular commits**. Do not bundle multiple features into one commit.
4.  **Mark Your Work:** When you complete a task in `ISSUES.md`, mark it as done and append your signature: `[x] Task Name (@yourusername YYYY-MM-DD HH:MM)`.

---

## 🔖 Proposed Names
- **StellarMarketID**
- **XMerchant**
- **StellarPresence**
- **XTradeID**
