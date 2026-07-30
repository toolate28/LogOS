#!/usr/bin/env python3
import os
import json
import hashlib
from datetime import datetime
from web3 import Web3

# Placeholder NEAR anchor script: constructs a Merkle root from today's ATOM trail
# and prepares a NEAR testnet transaction (requires near-api-py or NEAR CLI integration).
# This script only computes the Merkle root and prints a prepared 'anchor' payload.

ATOM_DIR = r"F:\Users\Matthew Ruhnau\LogOS\9P2000.L\atom-trail"
OUT_JSON = r"F:\Users\Matthew Ruhnau\LogOS\9P2000.L\coherence\atom_anchor_payload.json"

def sha256_hex(b):
    return hashlib.sha256(b).hexdigest()

def collect_atom_hashes(date_prefix=None):
    hashes = []
    if not os.path.isdir(ATOM_DIR):
        return hashes
    for fname in sorted(os.listdir(ATOM_DIR)):
        if date_prefix and not fname.startswith(date_prefix):
            continue
        path = os.path.join(ATOM_DIR, fname)
        if not os.path.isfile(path):
            continue
        with open(path, 'rb') as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                # hash the line content
                hashes.append(sha256_hex(line))
    return hashes

def merkle_root(hashes):
    if not hashes:
        return None
    level = [bytes.fromhex(h) for h in hashes]
    while len(level) > 1:
        if len(level) % 2 == 1:
            level.append(level[-1])
        next_level = []
        for i in range(0, len(level), 2):
            next_level.append(hashlib.sha256(level[i] + level[i+1]).digest())
        level = next_level
    return level[0].hex()

if __name__ == '__main__':
    # default: anchor today's entries
    today = datetime.utcnow().strftime('%Y-%m-%d')
    hashes = collect_atom_hashes(date_prefix=None)
    root = merkle_root(hashes)
    payload = {
        'timestamp': datetime.utcnow().isoformat() + 'Z',
        'count': len(hashes),
        'merkle_root': root,
        'note': 'Prepared anchor payload for NEAR testnet. Sign and submit using NEAR signer.'
    }
    with open(OUT_JSON, 'w') as f:
        json.dump(payload, f, indent=2)
    print('Prepared anchor payload at', OUT_JSON)
    print(json.dumps(payload, indent=2))
