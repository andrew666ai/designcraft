# Security

DesignCraft is a local page-layout application for personal use. It does not listen on the network unless you turn the control channel on, and that channel stays on loopback.

## Reporting a vulnerability

Give the maintainers a chance to look before you publish exploit details or a weaponized file. This repository has no private security inbox. Contact the repository owner, keep the public note short and non-exploitable, and agree a private way to share samples. Do not open a public issue with a working exploit, a secret, or a malicious attachment.

## Desktop use

Starting the app without `--control` does not open a port. The web build does not expose this control port. Normal editing, menus, and file dialogs are unchanged when automation is off.

## Control channel

`--control PORT` (or `DESIGNCRAFT_CONTROL_PORT`) binds **127.0.0.1** only. The first line on every connection must be a bearer-token handshake. No method runs before it succeeds:

```json
{"id": "auth", "method": "auth", "params": {"token": "<64 hexadecimal characters>"}}
```

A wrong token and a missing handshake both reply `authentication required` and close the connection. The token is 256 bits, from the operating system CSPRNG. Comparison accepts either hex case.

Supply it with `--control-token-file PATH` or `DESIGNCRAFT_CONTROL_TOKEN_FILE` (a missing file is created, mode `0600` on Unix). `--control-token` and `DESIGNCRAFT_CONTROL_TOKEN` also work; prefer a file so the token is not in the process list or the shell history. If you pass neither, DesignCraft prints a one-shot token to stderr for that launch. Do not commit tokens, paste them into chat, or put them in logs.

Budgets, per listener: 16 active connections, 1 MiB request lines, 8 MiB replies. Over the cap the reply is `connection limit reached`, `request exceeds … bytes`, or `response exceeds … bytes`. Idle connections time out after 30 seconds. There is no per-tool capability split: a client that has the token can call the whole control surface.

Do not tunnel or proxy this unencrypted protocol off the machine.

## MCP

Prefer stdio: `designcraft-cli mcp` (headless) speaks JSON-RPC on standard input and does not open a port or require a token. Request lines are still capped at 1 MiB.

`designcraft-cli mcp --connect`, `designcraft-cli script --connect`, and `designcraft-cli app` bridge to the desktop control port. They use the same token, refuse any non-loopback address, and send `auth` before any tool or command. Pass the token with the same flags or environment variables. Headless MCP and `designcraft-cli run` do not need a token.

## What this does not do

No capability model, session expiry, audit log, multi-tenant sessions, or workspace jail. An authenticated control client is trusted with the documents the app has open. Loopback TCP is not encrypted. This is not a sandbox against other programs on the same machine: a process that can read the token can drive the editor.
