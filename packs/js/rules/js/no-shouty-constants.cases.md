# js/no-shouty-constants — cases

### case: one use

```js
const FOO = "FOO";
console.log(FOO);
```

```expect
1:7 js/no-shouty-constants
```

```fixed
console.log("FOO");
```

### case: two uses

```js
const FOO = "FOO";
console.log(FOO, FOO);
```

```expect
(none)
```

### case: exported

```js
export const FOO = "FOO";
```

```expect
(none)
```

### case: a different value

```js
const FOO = "foo";
console.log(FOO);
```

```expect
(none)
```
