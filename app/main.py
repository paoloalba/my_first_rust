import os

import rust_app
rust_app.main()

from rust_app import UsState

print(type(UsState))
print(UsState.Alaska)
print(type(UsState.Alaska))

print("copying new rust binaries")
from shutil import copytree
copytree("/rust_app", "/home/rust_app", dirs_exist_ok=True)
print("copied new rust binaries")

print("main is done")