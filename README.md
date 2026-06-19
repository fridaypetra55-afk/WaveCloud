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
