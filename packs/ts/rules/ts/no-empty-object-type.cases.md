# ts/no-empty-object-type — cases

### case: a generic interface carries no edit

```ts
interface Box<T> {}
```

```expect
1:1 ts/no-empty-object-type
```

### case: an exported interface becomes a type

```ts
export interface Options {}
```

```expect
1:8 ts/no-empty-object-type
```

```fixed
export type Options = object;
```
