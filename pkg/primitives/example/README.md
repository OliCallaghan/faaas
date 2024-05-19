# Example Task Composition

```rs
fn handle(ctx: Context) -> Context {
    let foo = call("fn:one", ctx);
    let bar = call("fn:two", foo);

    block_on(foo, bar);

    let buz = call("fn:three");

    block_on(buz);

    if buz {
        call("fn:four", buz.ctx())
    } else {
        call("fn:five", buz.ctx())
    }
}

```

## Universe

A faaas universe is a deployment of a set of Tasks to a faaas cluster. The universe defines the atomic set of building blocks which workflows can be composed from.

The universe can be defined using the `@faaas/universe` package.

## Workflows

Workflows are compositions of Tasks. They can be represented as a directed cyclic graphs. These graphs are generated using the `@faaas/workflow` package.

## Task Context Protocol

Tasks need to pass around state using contexts.

In order to ensure separation of concerns, the interface for a context used to call function A will be different to that of function B. Since context is passed between function invocations
