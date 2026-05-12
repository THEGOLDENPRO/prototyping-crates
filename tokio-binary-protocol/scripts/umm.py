from subprocess import Popen

last_popen: Popen

for i in range(900):
    last_popen = Popen(["./target/debug/client"])

    print(f"client '{i}' done")

last_popen.wait()