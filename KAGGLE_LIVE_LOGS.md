# KAGGLE_LIVE_LOGS.md - reading a Kaggle kernel while it runs

How to watch a Kaggle kernel's log **live**, instead of waiting for the run to finish and
reading a post-mortem. Every recipe here was executed on 2026-10-28 against a real running
kernel; the measurements are in section 8, and nothing below is quoted from a tutorial.

**Audience:** anyone in this workspace driving Kaggle from PowerShell.
**Scope:** Kaggle CLI 2.2.4 on Windows. The API itself has no log endpoint - this is the CLI's
session-log channel, not a new API surface.

---

## 1. TL;DR

| Want | Command |
| --- | --- |
| Live stream | `kaggle kernels logs -f <owner>/<kernel>` |
| Live stream, keep a copy | `kaggle kernels logs -f <id> \| Tee-Object -FilePath .\live_log.txt` |
| Current state, no logs | `kaggle kernels status <id>` |
| Log after it ended | `kaggle kernels logs <id>` |
| Output files after it ended | `kaggle kernels output <id> -p .\out` |
| Remaining GPU hours | `kaggle quota` |

The CLI is **not** on `PATH` on this machine:

```powershell
. G:\Ai\.secrets\kaggle\env.ps1          # sets KAGGLE_API_TOKEN
$k = 'G:\Ai\.venv\Scripts\kaggle.exe'
```

---

## 2. Why this file exists

The working assumption in this workspace was that Kaggle only gives you logs after the run
finishes - `kernels output` is the only command any guide mentions, and it does nothing while
a kernel is running. That assumption was wrong, and it cost real time: a run died at 36 s on a
malformed `cmake` argv, and the failure was invisible until the session ended.

The live channel is `kaggle kernels logs -f`. It exists in CLI 2.2.4, it is one flag, and its
own help text says so:

```text
usage: kaggle kernels logs [-h] [-f] [kernel]

  kernel        Kernel URL suffix in format <owner>/<kernel-name>
  -f, --follow  Stream live execution logs from the running session (like tail -f)
```

Check your own CLI before trusting a guide:

```powershell
& $k kernels logs --help | Select-String 'follow'
```

If `--follow` is not listed, the installed version predates the feature.

---

## 3. Verified behaviour matrix

Measured against one running kernel and one finished kernel, 2026-10-28.

| Command | Run is RUNNING | Run is COMPLETE / ERROR |
| --- | --- | --- |
| `kernels logs -f <id>` | Streams. Replays the session from line 1, then follows. | Prints the log and exits. |
| `kernels logs <id>` | **1 line only** (the CLI banner). Effectively empty. | Prints the full log. |
| `kernels output <id> -p dir` | **Exit 0, 0 files.** Silent no-op. | Downloads the saved output. |
| `kernels status <id>` | `KernelWorkerStatus.RUNNING` | `COMPLETE` / `ERROR` |
| `kernels pull <id> -m` | Metadata only, no logs. | Same. |

Two consequences worth stating plainly:

1. **`kernels output` does not fail while a run is in progress.** It exits 0 and writes
   nothing, so "no error" is not "have data". Any polling loop must check the file count, or
   must gate on `kernels status` first.
2. **`kernels logs` without `-f` is the wrong tool mid-run**, and its near-empty output looks
   like a broken kernel rather than a broken command.

---

## 4. Recipes

### 4.1 Follow a run and keep the log

```powershell
. G:\Ai\.secrets\kaggle\env.ps1
$k = 'G:\Ai\.venv\Scripts\kaggle.exe'
$id = 'essencefromexistence/lfm3-1-7b-ternary-ladder'

& $k kernels logs -f $id | Tee-Object -FilePath .\live_log.txt
```

`Tee-Object` matters: `-f` output is otherwise lost when the terminal closes, and each new
invocation replays from the start anyway, so the file is the record.

### 4.2 Watch without blocking a session (background follower)

```powershell
$p = Start-Process -FilePath $k `
    -ArgumentList 'kernels','logs','-f',$id `
    -RedirectStandardOutput .\follow.txt `
    -RedirectStandardError  .\follow.err `
    -NoNewWindow -PassThru
# ... do other work, then read .\follow.txt
```

This is how an agent can "watch" a run without holding a terminal: the file grows while the
run is alive. Whether the follower exits by itself at completion has **not** been observed
yet; treat exit code as unreliable and use `kernels status` as the authority.

### 4.3 Detect a stall (the thing live logs are actually for)

A kernel that prints nothing for many minutes is indistinguishable from a wedged one unless
you measure the gap. Every line this workspace emits starts with a kernel-relative stamp:

```powershell
$stamps = Select-String -Path .\follow.txt -Pattern '^\[\s*([0-9.]+)s\]' |
    ForEach-Object { [double]$_.Matches[0].Groups[1].Value }
$last = ($stamps | Measure-Object -Maximum).Maximum
$wall = (Get-Date) - (Get-Item .\follow.txt).LastWriteTime
"last kernel stamp: ${last}s | file untouched for: $([int]$wall.TotalMinutes) min"
```

If the file has been untouched for longer than the heaviest step is expected to take, the run
is either in a silent subprocess (section 5) or wedged - and those two look identical from
outside.

### 4.4 Fetch results, correctly

```powershell
& $k kernels status $id                      # must say COMPLETE or ERROR
& $k kernels output $id -p .\out
if (-not (Get-ChildItem .\out -Recurse -File)) { 'no output files -- do not trust exit 0' }
```

---

## 5. The caveat that makes live logs look broken

Live logs stream **what the program flushes**. A wrapper that captures a child process hides
it. This workspace's own kernel helper did exactly that:

```python
# looks fine, prints nothing until the child exits
proc = subprocess.run(cmd, capture_output=True, text=True)
print("\n".join(out.splitlines()[-40:]))
```

During a 10-minute CUDA build the log therefore sat frozen on one `RUN cmake --build ...`
line, and the stream could not distinguish "compiling" from "hung". The fix is to iterate the
child's output as it arrives:

```python
proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                        text=True, bufsize=1)
tail = []
for line in proc.stdout:
    print(line, end="", flush=True)     # arrives in the Kaggle log immediately
    tail = (tail + [line])[-40:]
rc = proc.wait()
```

Rule of thumb: **if a live log stops moving, suspect `capture_output` before suspecting
Kaggle.**

---

## 6. Traps

| Trap | What happens | Guard |
| --- | --- | --- |
| `kernels output` while running | exit 0, zero files | check file count after download |
| `kernels logs` without `-f` while running | ~1 line of banner | always use `-f` mid-run |
| Each `-f` replays from line 1 | duplicate lines if you re-follow | dedupe by leading line number, or wipe the file |
| Kernel id slugified | `lfm3-1.7b-...` -> `lfm3-1-7b-...`; the dotted id makes a 2nd kernel | pull the live metadata |
| `--accelerator` unvalidated | forwarded verbatim to `machine_shape` | confirm `device_count` in the log |
| Silent subprocess | log freezes with no error | stream the child (section 5) |
| `-f` drops the connection | the stream prints `Log stream connection lost, reconnecting...` and replays from the start | dedupe by line number; do not treat repeats as kernel output |
| Session cap | GPU sessions die at the platform limit, not at your timeout | follow, do not assume |
| Output quota | `/kaggle/working` ~20 GiB; dying at save = nothing measured | scratch in `/kaggle/temp` |

---

## 7. Reuse checklist for any kernel

1. `& $k kernels status <id>` - is it even running?
2. `& $k kernels logs -f <id> | Tee-Object .\live_log.txt` - start the stream.
3. Read the first ~20 lines: confirm the machine, the GPU count and the commit/tag the run
   claims. A wrong accelerator is cheap to catch there and expensive later.
4. Watch for the first `Traceback`; a fast failure is usually an argv or path bug, not the
   science.
5. On completion: `kernels status` -> `COMPLETE`, then `kernels output`, then verify file count.

---

## 8. Receipts for every claim above

| Claim | Evidence |
| --- | --- |
| The subcommand and flag exist | `kaggle\cli.py:1382-1393`, quoted below |
| `-f` streams while RUNNING | 43 lines in 50 s, still streaming when killed |
| `-f` replays from line 1 | a second `-f` returned the same 43 lines from `0.00s` |
| `kernels logs` near-empty mid-run | 1 line returned while status was `RUNNING` |
| `kernels output` no-op mid-run | exit 0, 0 files, status still `RUNNING` |
| `--accelerator` skips validation | `kaggle_api_extended.py`: `request.machine_shape = acc` |
| 2 GPUs were actually granted | run log: `device_count: 2`, `fanout=True` |
| A live clip exists | fenced line below, taken mid-build |
| The live channel is SSE, not a polled file | `kaggle_api_extended.py:6871-6891` |
| `-f` reconnects and replays | observed `Log stream connection lost, reconnecting...` in a 50 s follower capture |
| Cancel is unreachable from the API | `kernels_api_service.py:139` needs `kernel_session_id`; status returns none |

The CLI definition, quoted verbatim (installed package path shortened):

```text
G:\Ai\.venv\Lib\site-packages\kaggle\cli.py:1382  # Kernels logs
:1392  "-f", "--follow", dest="follow", action="store_true",
       help=Help.param_kernel_logs_follow
help text: "Stream live execution logs from the running session (like tail -f)"
```

A live-stream clip, captured before the step it names had returned:

```text
[   17.8s] RUN cmake --build /kaggle/temp/ternary/llama.cpp/build -j 4
```

## 9. Cancelling a run (the CLI cannot, the UI can)

A run that is wasting quota is not stoppable from the CLI in 2.2.4: the `kernels`
subcommands are `list files init push pull output status logs delete topics`. `delete`
removes the whole kernel including the versions you want to keep.

The API *does* have the endpoint, but nothing returns the id it needs:

```text
POST /api/v1/kernels/cancel-session/{kernel_session_id}
kagglesdk/kernels/types/kernels_api_service.py:139
```

`ApiCancelKernelSessionRequest` takes exactly one field, `kernel_session_id` (int). The status
response carries only `status` and `failure_message`; the log-stream endpoint resolves a
session from `(user_name, kernel_slug, version_label)` server-side and never exposes the
numeric id. So a cancel call cannot be assembled from the public API alone.

What does work:

1. **Per-version status** - `kaggle kernels status <owner>/<slug>/2` reports that version,
   which is how you confirm which of two concurrent runs is the wasteful one.
2. **Stop in the web UI** - open the kernel, select the version, press Stop. This is the only
   supported way to end one run without touching the other.
3. **Pushing a new version does not cancel the old one.** Verified: v2 stayed `RUNNING` while
   v3 started, so two sessions ran concurrently and both billed. Push, then stop.

## 10. How the live channel actually works

Not a polled file. The CLI opens a server-sent-events stream:

```text
GET /api/v1/kernels/logs/stream/{owner}/{slug}
Accept: text/event-stream
kaggle_api_extended.py:6871-6891
```

While the session runs, that endpoint proxies the worker's SSE feed (`{stream_name, time,
data}` events, terminated by an `END_OF_LOG` sentinel). Once the session has finished it
returns the persisted log blob from object storage with a non-SSE content type, which is why
`kernels logs` and `kernels logs -f` behave differently depending on state - same URL, two
backends.

---

MIT OR Apache-2.0
updated: 2026-10-28 00:00 UTC
