Soroban contract for WaveCloud.

Build (locally with Docker):

```
cd contracts/soroban
docker build -t wavecloud-soroban .
```

The contract stores policies per-field in storage and emits `payout` and `insolvent` events when readings trigger payouts.

Notes:
- Token/XLM transfers are placeholders and should be integrated via Stellar token contract calls.
- Add more robust oracle signature verification and replay protection in production.
