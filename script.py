import os, sys

for day in range(1, 25):
    # Read base file
    with open("src/template.rs", "r") as f:
        content = f.read()

    # Create input .txt file
    with open(f"inputs/day{day}.txt", "w") as f:
        pass

    # Create src file
    with open(f"src/day{day}.rs", "w") as f:
        f.write(content.replace("REPLACEDAY", str(day)))