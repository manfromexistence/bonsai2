"""Benchmark the four single-model layers: decision, reasoning, OCR, vision.

Usage: python bench_core.py <decision|reason|ocr|vision>
Prints wall time, CPU (percent of the whole machine) and the model output.
"""
import json
import os
import subprocess
import sys
import threading
import time

import psutil

PY = r"G:\Dx\decisions\.venv-vision\Scripts\python.exe"
LLAMA_BENCH = r"G:\Temp\UserTemp\opencode\dx-fork-build\bin\llama-bench.exe"
MTMD = r"G:\Temp\UserTemp\opencode\vkb\bin\llama-mtmd-cli.exe"
DX = r"G:\Dx\decisions\build\dx-typesafe.exe"
OUT = r"G:\Temp\UserTemp\opencode\model-tests\out"
CORES = psutil.cpu_count(logical=True)


def run_measured(cmd, timeout=600, cwd=None):
    """Run a command, return (code, stdout, stderr, wall, avg_cpu_pct, peak_cpu_pct).

    CPU is exact: cumulative process CPU time / wall / logical threads.
    """
    t0 = time.time()
    proc = psutil.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                        text=True, encoding="utf-8", errors="replace", cwd=cwd)
    cpu_time = 0.0
    peak = 0.0
    while proc.poll() is None:
        try:
            ct = proc.cpu_times()
            cpu_time = ct.user + ct.system
            peak = max(peak, proc.cpu_percent(interval=None) / CORES)
        except psutil.Error:
            break
        if time.time() - t0 > timeout:
            proc.kill()
            break
        time.sleep(0.05)
    out, err = proc.communicate()
    try:
        ct = proc.cpu_times()
        cpu_time = max(cpu_time, ct.user + ct.system)
    except psutil.Error:
        pass
    wall = time.time() - t0
    return proc.returncode, out or "", err or "", wall, (cpu_time / wall / CORES * 100 if wall else 0), peak


def report(name, wall, avg, peak):
    print(f"RESULT {name} wall_s={wall:.2f} cpu_avg={avg:.0f}% cpu_peak={peak:.0f}% "
          f"(of {CORES} threads)", flush=True)


def reason():
    cmd = [LLAMA_BENCH, "-m",
           r"G:\Dx\models\reasoning\dx-reasoning-1.0.gguf",
           "-t", "6", "-ngl", "0", "-n", "256", "-p", "32", "-r", "3"]
    code, out, err, wall, avg, peak = run_measured(cmd, timeout=600)
    print(out[-2500:])
    print(err[-800:])
    report("reasoning", wall, avg, peak)


def ocr():
    code, out, err, wall, avg, peak = run_measured(
        [PY, r"G:\Temp\UserTemp\opencode\model-tests\test_ocr.py",
         r"G:\Temp\UserTemp\opencode\model-tests\ui_text.png"], timeout=300)
    print(out)
    print(err[-500:])
    report("ocr", wall, avg, peak)


def vision():
    image = r"G:\Temp\UserTemp\opencode\model-tests\ui_text.png"
    prompt = "What text is written in this image? Read it exactly."
    cases = {
        "visionpsy-q4_k_m": [
            r"G:\Dx\models\vlm\visionpsy-nano-460m-flash-q4_k_m-imat.gguf",
            r"G:\Dx\models\vlm\mmproj-visionpsy-nano-460m-flash-q8.gguf",
        ],
        "dx-vision-v1.0": [
            r"G:\Dx\zzz\dx-vision-v1.0\dx-vision-v1.0-Q4_K_M.gguf",
            r"G:\Dx\zzz\dx-vision-v1.0\dx-vision-v1.0-mmproj-f16.gguf",
        ],
    }
    for name, (model, mmproj) in cases.items():
        cmd = [MTMD, "-m", model, "--mmproj", mmproj, "--image", image,
               "-p", prompt, "-n", "64", "-t", "8", "--temp", "0"]
        code, out, err, wall, avg, peak = run_measured(cmd, timeout=600)
        print(f"--- {name} exit={code}")
        for ln in (out or "").splitlines():
            if ln.strip():
                print("OUT", ln)
        for ln in (err or "").splitlines():
            if "error" in ln.lower():
                print("ERR", ln)
        report(name, wall, avg, peak)


def decision():
    args = ["choice", "--state",
            "The launch window opens at 18:00 UTC and the client build must be signed before then.",
            "--instructions", "Pick the next action.",
            "--options",
            "sign:sign the build now,test:run the full suite again,ship:ship unsigned,wail:do nothing"]
    code, out, err, wall, avg, peak = run_measured([DX] + args, timeout=300)
    print("COLD OUT", out.strip()[-800:])
    print("COLD ERR", err.strip()[-300:])
    report("decision-cold", wall, avg, peak)
    # warm daemon
    subprocess.run([DX, "daemon", "start"], capture_output=True, text=True)
    time.sleep(1.5)
    for i in range(3):
        code, out, err, wall, avg, peak = run_measured([DX] + args, timeout=300)
        print(f"WARM{i} OUT", out.strip()[-300:])
        report(f"decision-warm{i}", wall, avg, peak)
    subprocess.run([DX, "daemon", "stop"], capture_output=True, text=True)


if __name__ == "__main__":
    globals()[sys.argv[1]]()
