import os
from shutil import copytree

src_dir = os.path.join(".", "permanent_storage", "copied_rust_build")
dst_dir = os.path.join(".", "docker_helpers")

# for eee in os.walk(src_dir):
#     print(eee)
#     break

copytree(src_dir, dst_dir, dirs_exist_ok=True)
