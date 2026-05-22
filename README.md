Project name:

CanteenCredit (Soroban Smart Meal System)



Name: Đặng Minh Hùng

First-year Information Technology student

Studying at Ho Chi Minh City University of Science (HCMUS)

Passionate about algorithms, data structures, and system optimization

Blockchain developer building practical dApps on Soroban/Stellar

Dedicated to solving real-world problems for the student community

Project details:

A digital meal token system built via smart contracts to replace physical meal tickets and cash at the university cafeteria. Administrators can issue credits to student wallets, and students can securely sign transactions to pay for their meals.

Vision:

To eliminate the friction of traditional cash transactions on campus, creating a transparent, fast, and fully digital ecosystem for the university.


Project Description:
CanteenCredit is a digital meal token system built on the Stellar Soroban blockchain, designed specifically for university ecosystems. It replaces physical meal tickets and cash with a seamless, smart contract-driven economy. Administrators can easily recharge student wallets with custom canteen tokens, while students use their digital wallets to pay for daily meals securely and instantly. By leveraging Soroban's fast and low-cost infrastructure, CanteenCredit eliminates long queues, prevents lost meal cards, and provides the university with transparent, real-time transaction data. This project modernizes campus dining, ensuring a frictionless and fully cashless experience for both students and cafeteria staff.

Vision Statement:
Our vision is to transform university campuses into fully integrated digital ecosystems, starting with the cafeteria. CanteenCredit aims to eliminate the friction of traditional cash transactions, creating a transparent, secure, and highly efficient daily routine for thousands of students. By normalizing blockchain utility in everyday student life, we aim to inspire a broader adoption of decentralized technologies across educational institutions, proving that smart contracts can have a massive, positive impact on local community economies.

Software Development Plan:

Contract Initialization & Setup: Define the DataKey enums (Admin, Balance). Implement the init function to store the canteen administrator's address on the Soroban ledger permanently.

Core State Variables & View Functions: Implement persistent storage for student balances. Develop the get_balance function to allow users and front-end interfaces to query current meal token balances.

Admin Functions (Recharging): Develop the recharge function. Implement admin.require_auth() to strictly verify that only the authorized canteen manager can add tokens to a student's address.

User Functions (Payment): Create the pay_meal function where students spend tokens. Require student.require_auth() for security, check for sufficient balance, and deduct the token amount from persistent storage.

Front-End Integration: Build a lightweight React/Next.js application integrating the Freighter wallet. Create an Admin dashboard for recharging accounts and a Student interface for checking balances and signing meal payment transactions.

Testnet Deployment & Testing: Compile the Rust contract to WebAssembly (.wasm), deploy it to the Stellar Testnet using the Soroban CLI or Stellar IDE, and run end-to-end tests validating the complete user flow.

Personal Story Summary:
As a first-year IT student at the Ho Chi Minh City University of Science, I've always been fascinated by how technology can optimize daily routines. Navigating campus life, I noticed the inefficiencies and long queues at the university canteen. Driven by my passion for coding and exploring smart contracts, I decided to build CanteenCredit. It is my way of applying programming logic to a real-world problem, aiming to make campus life a bit easier, faster, and more modern for my fellow students.

Draft GitHub README:
# CanteenCredit 🍔

A Soroban smart contract-based digital credit system for university cafeterias. Built on the Stellar network.

## Prerequisites
* [Rust](https://www.rust-lang.org/tools/install)
* [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
* A Stellar wallet (e.g., [Freighter](https://www.freighter.app/))

## Installation & Setup

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/yourusername/canteen-credit.git](https://github.com/yourusername/canteen-credit.git)
   cd canteen-credit

cargo build --target wasm32-unknown-unknown --release
cargo test
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/canteen_credit.wasm \
  --source <YOUR_ADMIN_WALLET_SECRET> \
  --network testnet




Usage
Interact with the contract using the Soroban CLI to call init, recharge, pay_meal, and get_balance functions. Ensure you sign transactions with the appropriate Admin or Student wallets.