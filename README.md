# FaaAS: Asynchronicity as a first class citizen within FaaS

Oli Callaghan (01508136)

## Requirements

- Node ^20.11.0
- Cargo ^1.73.0
- Nix

## Notes

In order to get a working FaaAS runtime, make sure that your functions are deployed.

```bash
# Start @faaas/faaastime.
pnpm start

# Once @faaas/faaastime has started, deploy the example universe to it.
pnpm deploy
```
