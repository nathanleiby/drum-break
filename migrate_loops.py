import os
import json

names = os.listdir("./res/loops")

print(names)
for name in names:
    if name.endswith(".json"):
        print("is json:", name)
        with open(name, 'r') as f:
            data = json.loads(f.read())
            data['voices']['ride'] = []
            with open(name, 'w') as outfile:
                json.dump(data, outfile, indent=2)




