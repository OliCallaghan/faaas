# Runtime Assumptions

**Target module only**: Ideally parse the code and tree-shake at compile time.
**No dynamic imports**: They only make sense when code splitting for browser, and not in FaaS contexts.
