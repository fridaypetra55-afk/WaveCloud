import time
from stellar_sdk import Keypair, StrKey
from nacl.signing import VerifyKey

from main import map_gps_to_region, sign_reading, verify_signature


def test_map_region():
    assert map_gps_to_region(0.1, 0.1) == map_gps_to_region(0.4, 0.6)


def test_sign_and_verify():
    # generate ephemeral key for test
    kp = Keypair.random()
    seed = kp.secret
    region = 'R_0_0'
    rainfall = 12.5
    ts = int(time.time())
    sig = sign_reading(region, rainfall, ts, seed=seed)
    pub = kp.public_key
    assert verify_signature(region, rainfall, ts, sig, pub)
