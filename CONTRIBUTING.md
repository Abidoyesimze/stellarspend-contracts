# Contributing to StellarSpend Contracts

Welcome to the StellarSpend Contracts repository! This document provides a high-level overview of our contract modules and their responsibilities to help new and existing contributors navigate the codebase efficiently.

## Module Ownership Map

Below is a map of every top-level module/crate in the workspace, along with a one-line description of its responsibility.

| Module/Crate | Responsibility |
|---|---|
| `access-control` | Manages roles and permissions for contract administration. |
| `activity-feed` | Tracks and retrieves user and contract activity logs. |
| `asset_control` | Handles operations and restrictions for platform assets. |
| `balance` | Manages querying and formatting account balances. |
| `batch-conversion` | Handles bulk currency conversions in a single transaction. |
| `batch-history` | Stores and retrieves historical data for batched operations. |
| `batch-notifications` | Dispatches alerts for batch process completions and errors. |
| `batch-payment` | Executes multiple payments to various recipients simultaneously. |
| `batch-payment-reminders` | Manages scheduled reminders for bulk payment operations. |
| `batch-rewards` | Distributes loyalty and promotional rewards to multiple users. |
| `batch-token-mint` | Mints platform tokens in bulk for initial distribution or rewards. |
| `batch-transfer` | Handles transferring assets to multiple accounts efficiently. |
| `batch-wallet-creation` | Provisions multiple user wallets in a single operation. |
| `benchmarks` | Contains performance and gas benchmarking tests for the workspace. |
| `budget` | Core module for setting and tracking spending budgets. |
| `budget-allocation` | Logic for distributing funds across different budget categories. |
| `budget-recommendations` | AI/Analytics-driven suggestions for user budgets. |
| `category-analytics` | Analyzes spending behavior based on categories. |
| `contract-upgrade/new_contract` | Test harness and implementation for upgraded contracts. |
| `contract-upgrade/old_contract` | Test harness for previous contract versions during upgrades. |
| `currency-conversion` | Core logic and rates for converting between different assets. |
| `escrow` | Manages funds locked for conditional release. |
| `fee` | Calculates and deducts transaction fees (See Known Overlaps). |
| `merchant-tagging` | Associates transactions with known merchants and categories. |
| `multi-currency-wallet` | Core wallet logic supporting multiple asset balances. |
| `pausable` | Implements circuit breaker functionality to pause contract operations. |
| `recurring-payment` | Manages subscriptions and automated periodic transfers. |
| `savings-goals` | Logic for users to lock funds towards specific financial targets. |
| `shared` | Common utilities, types, and constants used across multiple contracts. |
| `shared-budgets` | Logic for multi-user shared spending budgets. |
| `spending-categories` | Manages the taxonomy of categories for transactions. |
| `spending-limits` | Enforces daily/weekly/monthly limits on user withdrawals. |
| `stellarspend-contracts` | The main/entrypoint contract for the stellarspend protocol. |
| `stellarspend-fee` | Handles fee deductions specifically for stellarspend operations (See Known Overlaps). |
| `transaction-analytics` | Generates insights and metrics from transaction history. |
| `transaction-validation` | Rules for validating transaction limits, formats, and authenticity. |
| `transactional` | Core transactional logic (See Known Overlaps). |
| `transactions` | Alternative/duplicate transactional storage and logic (See Known Overlaps). |
| `transfer` | Handles single, peer-to-peer asset transfers. |
| `user` | Single user profile and data management (See Known Overlaps). |
| `users` | Bulk or alternative user profile management (See Known Overlaps). |
| `zk-verifier` | Zero-knowledge proof verification for privacy-preserving operations. |

### Known Overlaps & Duplications

We are currently tracking several overlapping or duplicate modules. Resolving these is out of scope for general feature development but they are tracked in the following issues:

- **Transactions**: `transactional` vs `transactions` (Tracked in **Issue #707**)
- **Users**: `user` vs `users` (Tracked in separate issue)
- **Fees**: `fee` vs `stellarspend-fee` (Tracked in separate issue)
- **Budgets**: Reconciling duplicate entry points in `budget` vs `budget-*` crates (Tracked in **Issue #764**)

*Note: Please do not attempt to resolve these overlaps unless you are assigned to the specific tracking issue.*
