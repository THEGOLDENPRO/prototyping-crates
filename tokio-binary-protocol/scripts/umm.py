import sys
from subprocess import Popen

last_popen: Popen

for i in range(100):
    last_popen = Popen(["./target/debug/client"])

    sys.stdout.write(f"client '{i}' done \n")

last_popen.wait()