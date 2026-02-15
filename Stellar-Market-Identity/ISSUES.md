# Project Roadmap & Task Tracker 🚦

This document tracks all pending and completed work for **Stellar Market Identity**.

### 📝 Contribution Rule
When you complete a task:
1.  Change `[ ]` to `[x]`.
2.  Append your GitHub username and timestamp.
    *   **Example:** `[x] Implement Hello World contract (@thebabalola 2023-10-27 14:30)`

---

## 🏗️ Phase 1: Smart Contracts (Soroban)

### Core Identity Logic
- [ ] **Define Identity Struct:** Create a struct to hold business data (hash, category, reputation_score, updated_at).
- [ ] **Registration Function:** Implement `register_merchant(name: Symbol, category: Symbol)` function.
- [ ] **Reputation Update Logic:** Create an admin-only function `update_score(merchant: Address, new_score: u32)` for the AI oracle.
- [ ] **Getters:** Implement `get_identity(merchant: Address)` view function.

### Access Control
- [ ] **Admin Pattern:** Implement basic owner/admin control for updating scores.
- [ ] **Events:** Emit `MerchantRegistered` and `ScoreUpdated` events for the indexer.

### Testing
- [ ] **Unit Tests:** Write tests for registration flow.
- [ ] **Auth Tests:** Verify only admin can update scores.

---

## 💻 Phase 2: Frontend (Merchant Dashboard)

### Wallet Integration
- [ ] **Freighter Hook:** Implement `useFreighter` hook (see `GUIDE.md`).
- [ ] **Connection UI:** Build a visually appealing "Connect Wallet" button in the Navbar.

### Dashboard UI
- [ ] **Profile Card:** Display merchant name, category, and current reputation score.
- [ ] **Transaction History:** (Mockup) Table showing recent qualifying transactions.
- [ ] **Registration Form:** Form to call `register_merchant` for new users.

### Theming & UX
- [ ] **Dark/Light Mode:** Verify the toggle works correctly with the new branding.
- [ ] **Responsive Design:** Ensure dashboard works on mobile (critical for street vendors).

---

## 🧠 Phase 3: AI & Data Infrastructure

### Ingestion
- [ ] **Transaction Watcher:** Setup a script to listen for Stellar transactions involving registered merchant addresses.
- [ ] **Database Schema:** Design PostgreSQL schema for `transactions` and `merchants`.

### Scoring Model
- [ ] **Feature Engineering:** Define what metrics matter (transaction frequency, volume, account age).
- [ ] **Scoring Algorithm:** Implement a basic linear weighting model (v1) before moving to ML.

---

## 📋 General / Documentation
- [ ] **Audit:** Review contract for basic security flaws.
- [ ] **Demo Video:** Create a walkthrough of the registration process.
