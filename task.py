import subprocess
import sys
import os

APPS_DIR = "apps"
EMU_CLI_DIR = "emu-cli"
LOG_VIEWER_DIR = "frisc-log-viewer"


def run_cmd(cmd: str, dir: str = "./", ignore_error: bool = False):
    print(f"\033[32m{cmd}\033[0m")
    cp = subprocess.run(cmd, shell=True, cwd=dir)

    if cp.returncode != 0 and not ignore_error:
        print(f"returncode: {cp.returncode}")
        exit(0)


# tasks
def task_build_apps():
    d = f"./{APPS_DIR}"
    dirs = [f for f in os.listdir(d) if os.path.isdir(os.path.join(d, f))]

    for dir_name in dirs:
        pwd = f"{d}/{dir_name}"

        run_cmd("make clean", dir=pwd)
        run_cmd("make", dir=pwd)


def task_run_test():
    run_cmd("cargo test")


def task_run_log_viewer():
    run_cmd("npm run dev", dir=f"./{LOG_VIEWER_DIR}")


TASKS = [task_build_apps, task_run_test, task_run_log_viewer]

if __name__ == "__main__":
    args = sys.argv

    if len(args) > 1:
        for task in TASKS:
            if task.__name__ == args[1]:
                task()
                exit(0)

        print("Invalid task name.")

    else:
        print(f"Usage: {list(map(lambda x: x.__name__, TASKS))}")
