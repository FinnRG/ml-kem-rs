def mlkem768_generate() -> tuple[bytes, bytes]:
    """Generates a new ML-KEM 768 keypair.

    Returns:
        A tuple of (decapsulation_key, encapsulation_key) as bytes.
    """
    ...

def mlkem768_encapsulate(encapsulation_key: bytes) -> tuple[bytes, bytes]:
    """Encapsulates using the given encapsulation key.

    Args:
        encapsulation_key: The encapsulation key bytes.

    Returns:
        A tuple of (encapsulated_key, shared_secret) as bytes.

    Raises:
        ValueError: If the encapsulation key is invalid.
    """
    ...

def mlkem768_decapsulate(decapsulation_key: bytes, encapsulated_key: bytes) -> bytes:
    """Decapsulates using the given decapsulation key and encapsulated key.

    Args:
        decapsulation_key: The decapsulation key bytes.
        encapsulated_key: The encapsulated key bytes (must be 1088 bytes).

    Returns:
        The shared secret as bytes.

    Raises:
        ValueError: If the decapsulation key or encapsulated key is invalid.
    """
    ...
