# Notes from conversations

## Additional Background Topics to Cover

- How hypervisors operate?
- KVM Overview
  - Virtio devices
- Firecracker
- Kata Containers
- NEMU (Intel cleaning up QEMU)
- OSv
- Unikernels generally
- Docker containers, CGroups, Namespaces
- WASM encapsulation
- Tokio (green threads) vs OS level threading and processes

- Delimited continuations and effect types (WASMFX)

## Additional Approaches to Consider

- Use SWC to rewrite await statements to break and then use pubsub and an extra HTTP layer around that pushes and waits on responses etc.
