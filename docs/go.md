# Go — using the subscription outside the TUI

The Go key is a normal API key. The TUI is one client; anything that
speaks OpenAI / Anthropic / Gemini HTTP can be another. A raw call
costs exactly prompt + output — no agent baseline, no tool schemas, no
history — so one-shot jobs belong in a script, not in a session.

## Setup (once)

1. Console -> Go subscription -> copy the API key (`oc_sk_...`).
2. Keep it in the shell only, never in the repo:
   ` $env:OPENCODE_GO_API_KEY = "oc_sk_..."`.
3. Smoke test: `bun scripts/go-ask.mjs "say OK"`.

## Endpoints

Base: `https://opencode.ai/inference/go` (override: `GO_BASE_URL`).
Live model list: `https://opencode.ai/inference/go/v1/models`.

| Family | Path | Models (examples) |
| --- | --- | --- |
| Chat completions | `/openai/v1/chat/completions` | beam-free, kimi-k3, deepseek-v4.1-flash, glm-5.3, mimo, minimax-m3.1-flash |
| Responses | `/openai/v1/responses` | muse-spark-1.3-contributor, grok-4.7, gpt-6-luna |
| Messages | `/anthropic/v1/messages` | claude-haiku-5-5, minimax-m3, qwen3.8-max |

`scripts/go-ask.mjs` routes by model id automatically (`--api`
overrides). Auth is always `Authorization: Bearer $env:OPENCODE_GO_API_KEY`.

## Recipes

```powershell
# commit message for staged work (no session, ~100 tokens each way)
git diff --cached | bun scripts/go-ask.mjs --system "one conventional-commit line, imperative, no body"

# translate a file, Bangla default per house rule
Get-Content notes.en.txt | bun scripts/go-ask.mjs --system "translate to Bangla, keep markdown intact"

# review before pushing
git diff HEAD | bun scripts/go-ask.mjs --model kimi-k2.7-code --system "list bugs and risks as bullets, else say clean"

# strong model, one hard question (Responses family — routing is automatic)
bun scripts/go-ask.mjs --model muse-spark-1.3-contributor "why does <X> deadlock under <Y>?"
```

## TUI vs script

| Job | Where |
| --- | --- |
| One question, one file, one diff | script |
| Multi-step build, drag chains, cross-file edits | TUI session (fresh, one change) |
| Explore a codebase | `explore` subagent on beam-free (already the `small_model`) |
| Titles, compaction, summaries | automatic on `small_model`, costs nothing extra |

## Rules

- The key never enters git: not in scripts, docs, `.env`, or screenshots.
  `PROMPTS.md` is ignored for exactly this class of mistake — do not
  repeat it.
- Allowance is shared across TUI + scripts: a script call is cheap, not
  free. Batch trivially small calls instead of spraying them.
- Free-tier chat models 403 outside OpenCode; Go models do not. If a
  call 403s, the model id is wrong for the plan, not the key.

MIT OR Apache-2.0
updated: 2026-10-26 00:00 UTC
