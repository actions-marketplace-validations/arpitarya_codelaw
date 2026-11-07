# js/consistent-tuple-labels — cases

### case: an unlabelled rest element

```ts
type Route = [name: string, ...string[]];
```

```expect
1:14 js/consistent-tuple-labels
```

### case: a labelled rest element stays

```ts
type Route = [name: string, ...segments: string[]];
```

```expect
(none)
```

### case: no labels stays

```ts
type Point = [number, number];
```

```expect
(none)
```
