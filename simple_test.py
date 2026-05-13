#!/usr/bin/env python3
import base64
import socket

def get_machine_key():
    hostname = socket.gethostname()
    return hostname.encode('utf-8')

def encrypt_api_key(api_key):
    ENCRYPTION_MAGIC = b'GHOSTWRITER_V1'
    machine_key = get_machine_key()
    encrypted = bytearray()
    for i, b in enumerate(api_key.encode('utf-8')):
        encrypted.append(b ^ machine_key[i % len(machine_key)])
    result = ENCRYPTION_MAGIC + bytes(encrypted)
    return base64.b64encode(result).decode('utf-8')

def decrypt_api_key(encrypted_b64):
    ENCRYPTION_MAGIC = b'GHOSTWRITER_V1'
    data = base64.b64decode(encrypted_b64)
    machine_key = get_machine_key()

    if data.startswith(ENCRYPTION_MAGIC):
        encrypted_bytes = data[len(ENCRYPTION_MAGIC):]
        decrypted = bytearray()
        for i, b in enumerate(encrypted_bytes):
            decrypted.append(b ^ machine_key[i % len(machine_key)])
        return decrypted.decode('utf-8')
    else:
        raise ValueError("Invalid encrypted key format")

if __name__ == "__main__":
    api_key = "YOUR_API_KEY_HERE"
    print(f"API key: {api_key}")
    print(f"Hostname: {socket.gethostname()}")

    encrypted = encrypt_api_key(api_key)
    print(f"Encrypted: {encrypted}")

    decrypted = decrypt_api_key(encrypted)
    print(f"Decrypted: {decrypted}")
    print(f"Match: {decrypted == api_key}")

    config_encrypted = "R0hPU1RXUklURVJfVjEeCk4NHUIdH0FeAVkNX1dQUlpYCB4KXQcEXwxZBlFdX10dCF9RAllUBVBVXA0ISgkLVFYPVQUHAwpeDUtZDQZZDQlZU1ELWVxP"
    print(f"\nConfig encrypted: {config_encrypted}")
    try:
        decrypted_from_config = decrypt_api_key(config_encrypted)
        print(f"Decrypted from config: {decrypted_from_config}")
    except Exception as e:
        print(f"Error decrypting config key: {e}")
