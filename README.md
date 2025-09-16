Tiny OpenAI Curl Client

This repo contains a teeny tiny curl-based client that reads auth from `~/.codex/auth.json` and calls OpenAI's Chat Completions API.

Requirements
- `bash`, `curl`, and `python3` available in your `$PATH`.
- Auth file at `~/.codex/auth.json` with one of these structures:
  - `{ "openai": { "api_key": "sk-...", "organization": "org_...", "api_base": "https://api.openai.com" } }`
  - `{ "api_key": "sk-...", "org": "org_...", "api_base": "https://api.openai.com" }`
  - `{ "openai_api_key": "sk-..." }` or `{ "token": "sk-..." }`

Install
No installation needed; run directly from the repo:

`bin/tiny-openai-curl --help`

Usage
- Quick prompt:
  `bin/tiny-openai-curl "Say hello in one sentence."`

- Pipe input:
  `echo "Write a haiku about shells" | bin/tiny-openai-curl`

- Choose model and system prompt:
  `bin/tiny-openai-curl -m gpt-4o-mini -s "You are terse." "Summarize: The quick brown fox..."`

- Raw JSON response:
  `bin/tiny-openai-curl --raw "Return the JSON only."`

Notes
- Default API base is `https://api.openai.com/v1` (derived from `api_base` if provided).
- If the response includes an error, a concise error message prints; use `--raw` to see the full JSON.
- This is a minimal proof-of-concept intended to be bolted onto LangChain later.

