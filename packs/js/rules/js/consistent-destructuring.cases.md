# js/consistent-destructuring — cases

### case: another property

```js
const { a } = o;
f(o.b);
```

```expect
(none)
```

### case: a call in between

```js
const { a } = o;
mutate(o);
const v = o.a;
```

```expect
(none)
```

### case: renamed

```js
const { a: first } = o;
const v = o.a;
```

```expect
2:11 js/consistent-destructuring
```

```fixed
const { a: first } = o;
const v = first;
```
