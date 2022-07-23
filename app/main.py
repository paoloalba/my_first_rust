import rust_app
rust_app.main()

from rust_app import UsState

print(type(UsState))
print(UsState.Alaska)
print(type(UsState.Alaska))

import os
from shutil import copytree

copytree(os.path.join("/rust_inst"), os.path.join(os.getenv("PMT_STG_PATH"), "copied_rust_build"), dirs_exist_ok=True)

print("main is done")