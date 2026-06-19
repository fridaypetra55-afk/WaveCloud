WaveCloud Oracle (Python)

This service fetches weather data from OpenWeatherMap, maps GPS coordinates to region buckets,
signs readings with the oracle's Stellar key, and submits them to the backend for relay to Soroban.

Quick start:

1. Copy `.env.example` to `.env` and fill in `OPENWEATHER_KEY` and `ORACLE_SIGNER_SEED`.
2. Run locally:

```bash
pip install -r requirements.txt
python main.py
```

Or build the Docker image:

```bash
docker build -t wavecloud-oracle:local .
docker run --env-file .env wavecloud-oracle:local
```
