# DX Website

`website` is a pnpm/Turborepo workspace for the shadcn/ui component library
and documentation website. It contains applications, packages, templates,
registry tooling, examples, and release configuration.

## Position

The website workspace is a frontend and component-registry surface. It is
related to DX Mobile but is a separate repository folder with its own package
scripts and dependency graph.

## Repository shape

- `apps/`: website applications.
- `packages/`: reusable packages and the shadcn CLI surface.
- `templates/`: starter templates.
- `scripts/`: registry and synchronization automation.
- `package.json`: pnpm/Turborepo commands.

## Validation

Run from `G:\\Dx\\website`:

```powershell
pnpm install
pnpm lint
pnpm typecheck
pnpm format:check
pnpm build
```

The workspace also defines registry validation and application-specific test
commands such as `pnpm validate:registries` and `pnpm test:apps`.

## Status boundary

A successful package install does not prove every application or registry target
builds. Validate the specific app or package being released.
