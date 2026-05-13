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
