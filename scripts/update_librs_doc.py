"""Update the content of README.md to src/lib.rs
"""

import os

os.chdir(os.path.dirname(os.path.dirname(__file__)))

src = "README.md"
dst = "src/lib.rs"

if __name__ == "__main__":
    readme_formatted = ""
    with open(src) as f:
        for line in f.readlines():
            readme_formatted += "//! "
            readme_formatted += line
    res = ""
    with open(dst) as f:
        before = True
        deleting = False
        inserted = False
        lines = f.readlines()
        i = 0
        while lines[i][:3] != "//!":
            res += lines[i]
            i += 1
        while lines[i][:3] == "//!":
            i += 1
        res += readme_formatted
        while i < len(lines):
            res += lines[i]
            i += 1
    with open(dst, "w") as f:
        f.write(res)

