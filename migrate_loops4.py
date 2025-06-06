"""
To run:

python3 migrate_loops4.py
"""


import os
import json
import uuid

prefix = "./assets/loops"
names = os.listdir(prefix)

# This migration adds
#    `id = "<uuid v4>"`
#    `name = "<filename without .json>"`

print(names)
for name in names:
    if name.endswith(".json"):
        print("is json:", name)
        name_no_ext = name.split(".json")[0]
        full_path = os.path.join(prefix, name)
        with open(full_path, 'r') as f:
            data = json.loads(f.read())
            data['length_in_beats'] = 16
            with open(full_path, 'w') as outfile:
                json.dump(data, outfile, indent=2)




