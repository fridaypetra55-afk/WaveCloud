# WaveCloud

Parametric weather insurance on Stellar + Soroban.

This repo contains a scaffold for WaveCloud — a protocol that issues automated XLM payouts
when oracle-verified rainfall or drought conditions meet farmers' policy thresholds.

Contents:
- `contracts/soroban` — Soroban smart contract skeletons for policy management, oracle verification, and pool handling.
- `oracle/python` — Python oracle service to fetch weather data and submit signed readings to Soroban.
- `backend/nestjs` — NestJS + PostgreSQL backend placeholder for indexing policies and analytics.
- `dashboard/nextjs` — Next.js UI placeholder for farmers to create policies and view payouts.
- `infra` — Docker Compose for local services (Postgres, Soroban RPC).
- `.github/workflows` — CI skeleton.

Next steps:
- Implement full Soroban contract serialization and secure token transfers.
- Implement NestJS API, DB migrations, and indexer.
- Build Next.js dashboard and integrate with Stellar wallets.
- Harden oracle authentication, replay protection, and pool solvency checks.
- # WaveCloud

**Parametric weather insurance on Stellar + Soroban.**

WaveCloud is a protocol that issues automated XLM payouts to farmers when oracle-verified rainfall or drought conditions cross the thresholds set in their policy. No claims process, no adjusters — if the on-chain weather data says the drought happened, the contract pays out.

> ⚠️ **Status: early scaffold.** This repo currently contains skeletons and placeholders across all services. It is not production-ready — see [Roadmap](#roadmap) below.

---

## How it works

1. A farmer creates a **policy** defining a location, a weather threshold (e.g. "rainfall below X mm in Y days"), and a payout amount.
2. The **oracle service** periodically fetches weather data for that location and submits signed readings on-chain.
3. The **Soroban contract** checks incoming readings against active policies.
4. If a threshold is breached, the contract automatically releases an **XLM payout** to the farmer's wallet — no manual claim required.

```
Farmer  ──▶  Dashboard (Next.js)  ──▶  Policy created on-chain
                                              │
Weather data ──▶ Oracle (Python) ──▶ Signed reading ──▶ Soroban contract
                                              │
                                   Threshold breached? ──▶ Automatic XLM payout
```

---

## Repository structure

| Path | Description |
|---|---|
| `contracts/soroban` | Soroban smart contract skeletons for policy management, oracle verification, and liquidity pool handling. |
| `oracle/python` | Python oracle service that fetches weather data and submits signed readings to the Soroban contracts. |
| `backend/nestjs` | NestJS + PostgreSQL backend for indexing policies, payouts, and analytics. |
| `dashboard/nextjs` | Next.js UI for farmers to create policies, connect wallets, and view payout history. |
| `infra` | Docker Compose setup for local development (Postgres, Soroban RPC, supporting services). |
| `.github/workflows` | CI pipeline skeleton. |

---

## Tech stack

- **Smart contracts:** [Soroban](https://soroban.stellar.org/) (Rust) on Stellar
- **Oracle service:** Python
- **Backend:** NestJS, PostgreSQL
- **Frontend:** Next.js
- **Infra:** Docker Compose

---

## Getting started

> These are placeholder instructions for the current scaffold — expect rough edges.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) + the [Soroban CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup)
- [Python](https://www.python.org/) 3.10+
- [Node.js](https://nodejs.org/) 18+
- [Docker](https://www.docker.com/) and Docker Compose

### Local setup

```bash
# Clone the repo
git clone https://github.com/fridaypetra55-afk/WaveCloud.git
cd WaveCloud

# Spin up local services (Postgres, Soroban RPC, etc.)
cd infra
docker compose up -d

# Build the Soroban contracts
cd ../contracts/soroban
cargo build --target wasm32-unknown-unknown --release

# Set up the oracle service
cd ../../oracle/python
pip install -r requirements.txt

# Set up the backend
cd ../../backend/nestjs
npm install
npm run start:dev

# Set up the dashboard
cd ../../dashboard/nextjs
npm install
npm run dev
```

*(Exact commands will evolve as each service moves past scaffold stage — check each subfolder for its own README once available.)*

---

## Roadmap

- [ ] Implement full Soroban contract serialization and secure token transfers
- [ ] Implement NestJS API, database migrations, and indexer
- [ ] Build out the Next.js dashboard and integrate Stellar wallet support
- [ ] Harden oracle authentication and add replay protection
- [ ] Add liquidity pool solvency checks
- [ ] CI: test coverage and contract verification in `.github/workflows`

---

## Contributing

Contributions are welcome. Good places to start:

- **Contracts:** policy lifecycle logic, oracle signature verification, pool accounting (`contracts/soroban`)
- **Oracle:** weather data source integrations, signing, retry/failure handling (`oracle/python`)
- **Backend:** indexing, API endpoints, analytics queries (`backend/nestjs`)
- **Frontend:** wallet integration, policy creation flow, payout history UI (`dashboard/nextjs`)
- **Docs & testing:** setup guides, contract test coverage, CI pipelines

Open an issue describing what you'd like to work on before submitting a large PR, so it can be scoped and discussed first.

---

## License

No license has been published yet for this repository. Until one is added, all rights are reserved by the author.

## Disclaimer

WaveCloud is experimental software dealing with financial payouts and on-chain funds. It has not been audited. Do not use in production or with real funds until contracts have been independently reviewed.
