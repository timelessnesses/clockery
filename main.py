import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--framecap", type=int, help="Limiting amount of frames being rendered (Default is unlimited)", default=0)

parsed = parser.parse_args()

import src

frame_cap = parsed.framecap

src.run(frame_cap)