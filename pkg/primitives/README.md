# Workflow orchestration primitives

## Task

Task<I, O>

A task is a workflow task which accepts an input, and returns an output.

## Proxy

Wraps a workflow, accepting an HTTP request and responding with an HTTP response.

## Linear

This is a WASM component that can be used to chain tasks

## Conditional

This is a WASM component that can be used to conditionally execute two branches.

```rs
fn handle(req: Request) -> Response {
    let foo = call("fn:one");
    let bar = call("fn:two");

    block_on(foo, bar);

    let buz = call("fn:three");

    if buz {
        for
    }
}

```
