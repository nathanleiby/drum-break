"""
To run:

python3 migrate_loops2.py
"""


import os
import json

prefix = "./assets/loops"
names = os.listdir(prefix)

# This migration adds `voices.crash = []`

print(names)
for name in names:
    if name.endswith(".json"):
        print("is json:", name)
        full_path = os.path.join(prefix, name)
        with open(full_path, 'r') as f:
            data = json.loads(f.read())
            data['voices']['crash'] = []
            with open(full_path, 'w') as outfile:
                json.dump(data, outfile, indent=2)




