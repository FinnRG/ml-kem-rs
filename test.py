from ml_kem_rs import mlkem768_generate, mlkem768_encapsulate, mlkem768_decapsulate

dk, ek = mlkem768_generate()
ciphertext, ss1 = mlkem768_encapsulate(ek)
ss2 = mlkem768_decapsulate(dk, ciphertext)

print(ss1 == ss2)