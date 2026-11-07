# js/consistent-template-literal-escape — cases

### case: both characters escaped

```js
const t = `\$\{x}`;
```

```expect
1:11 js/consistent-template-literal-escape
```

```fixed
const t = `\${x}`;
```

### case: the brace escaped

```js
const t = `a $\{x} b`;
```

```expect
1:11 js/consistent-template-literal-escape
```

```fixed
const t = `a \${x} b`;
```

### case: the dollar escaped stays

```js
const t = `\${x} and ${y}`;
```

```expect
(none)
```
