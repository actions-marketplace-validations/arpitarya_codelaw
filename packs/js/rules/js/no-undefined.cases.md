# js/no-undefined — cases

### case: the `undefined` type in TypeScript is left alone (W-139 corpus)

````ts
let result: string | undefined;
type Missing = undefined;
````

```expect
(none)
```

### case: the value in TypeScript is still reported

````ts
let result: string | undefined = undefined;
````

```expect
1:34 js/no-undefined
```

````fixed
let result: string | undefined = void 0;
````
