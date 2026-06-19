def map_gps_to_region(lat, lon):
import os
import time
import requests
from dotenv import load_dotenv
from stellar_sdk import Keypair, StrKey
from nacl.signing import SigningKey, VerifyKey
from nacl.encoding import HexEncoder

load_dotenv()

OPENWEATHER_KEY = os.getenv('OPENWEATHER_KEY')
BACKEND_URL = os.getenv('BACKEND_URL', 'http://localhost:3000')
ORACLE_SIGNER_SEED = os.getenv('ORACLE_SIGNER_SEED')

KEYPAIR = Keypair.from_secret(ORACLE_SIGNER_SEED) if ORACLE_SIGNER_SEED else None


def map_gps_to_region(lat, lon):
    # deterministic region bucket: 0.5 degree grid (example)
    lat_bucket = int(lat * 2)
    lon_bucket = int(lon * 2)
    return f"R_{lat_bucket}_{lon_bucket}"


def fetch_rain(lat, lon):
    if not OPENWEATHER_KEY:
        raise Exception('OPENWEATHER_KEY not set')
    url = f"https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={OPENWEATHER_KEY}&units=metric"
    r = requests.get(url, timeout=10)
    r.raise_for_status()
    data = r.json()
    rain = 0.0
    if 'rain' in data:
        rain = data['rain'].get('1h', data['rain'].get('3h', 0.0))
    return float(rain), data


def sign_reading(region: str, rainfall_mm: float, timestamp: int, seed: str = None) -> str:
    if seed is None:
        if not ORACLE_SIGNER_SEED:
            raise Exception('No oracle signer configured')
        seed = ORACLE_SIGNER_SEED
    seed_bytes = StrKey.decode_ed25519_secret_seed(seed)
    sk = SigningKey(seed_bytes)
    payload = f"{region}|{rainfall_mm:.6f}|{timestamp}".encode()
    sig = sk.sign(payload).signature
    return sig.hex()


def verify_signature(region: str, rainfall_mm: float, timestamp: int, signature_hex: str, pubkey: str) -> bool:
    payload = f"{region}|{rainfall_mm:.6f}|{timestamp}".encode()
    vk_bytes = StrKey.decode_ed25519_public_key(pubkey)
    vk = VerifyKey(vk_bytes)
    try:
        vk.verify(payload, bytes.fromhex(signature_hex))
        return True
    except Exception:
        return False


def submit_reading(region, rainfall_mm, timestamp, signature_hex, public_key=None):
    if public_key is None:
        public_key = KEYPAIR.public_key() if KEYPAIR else None
    payload = {
        'region': region,
        'rainfall_mm': rainfall_mm,
        'timestamp': timestamp,
        'signature': signature_hex,
        'oracle_address': public_key,
    }
    r = requests.post(f"{BACKEND_URL}/api/oracle/submit", json=payload, timeout=10)
    return r.status_code, r.text


if __name__ == '__main__':
    lat = float(os.getenv('TEST_LAT', '0'))
    lon = float(os.getenv('TEST_LON', '0'))
    region = map_gps_to_region(lat, lon)
    rain, raw = fetch_rain(lat, lon)
    ts = int(time.time())
    sig = sign_reading(region, rain, ts)
    print('Submitting', region, rain, ts, sig[:16])
    status, text = submit_reading(region, rain, ts, sig)
    print(status, text)
