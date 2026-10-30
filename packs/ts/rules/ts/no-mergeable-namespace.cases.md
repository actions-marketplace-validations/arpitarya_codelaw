# ts/no-mergeable-namespace — cases

### case: exported and ambient namespaces

```ts
export namespace Api { export const a = 1; }
declare namespace Other {}
export namespace Api { export const b = 2; }
```

```expect
3:8 ts/no-mergeable-namespace
```

### case: module augmentations of one path

```ts
declare module "./components" { export const a: number; }
declare module "./components" { export const b: number; }
```

```expect
```

### case: a namespace merged with a class is a different thing

```ts
class Api {}
namespace Api { export const a = 1; }
```

```expect
```
