import hashlib
import sys

if len(sys.argv) != 2:
    print("Kullanım: python hash_checker.py <dosya_yolu>")
    sys.exit(1)

file_path = sys.argv[1]

try:
    with open(file_path, 'rb') as f:
        file_bytes = f.read()
        sha256_hash = hashlib.sha256(file_bytes).hexdigest()
        print(f"Dosya: {file_path}")
        print(f"SHA256 Hash: {sha256_hash}")
except FileNotFoundError:
    print(f"Hata: Dosya bulunamadı -> {file_path}")