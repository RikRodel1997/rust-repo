import random
import time

time.sleep(30)
print("Hello from Python")

while True:
    is_true = random.choice([True, True, False])

    if is_true:
        continue
    else:
        raise Exception("OH NO")
