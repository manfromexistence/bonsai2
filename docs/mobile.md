# DX Mobile

`mobile` is the DX cross-platform application workspace. It combines a Next.js
web application, Tauri v2 desktop/mobile targets, Bun workspaces, Turborepo,
Drizzle/Postgres services, and AI SDK integrations.

## Position

DX Mobile is the application surface for ChatGPT-, v0-, and Claude-like
experiences together with the shadcn/ui component registry. It is an application
workspace, not one of the Rust CLI engines.

## Repository shape

- `apps/` and `packages/`: Bun/Turborepo workspace projects.
- `src/`: application and shared source code.
- `src-tauri/`: Tauri native host integration.
- `registry.json`: generated component-registry output.
- `package.json`: workspace scripts and runtime dependencies.

## Validation

Run from `G:\\Dx\\mobile`:

```powershell
bun install
bun run lint
bun run typecheck
bun run test
bun run build
```

The repository instructions identify `registry.json` and other registry outputs
as generated files; update registry source and run the registry build instead of
editing generated output directly.

## Status boundary

Presence of dependencies or generated output does not prove that web, Tauri,
desktop, and mobile targets all build. Each target needs its own current build
receipt.
