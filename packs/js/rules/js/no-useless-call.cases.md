# js/no-useless-call — cases

### case: a different this is a real call

```js
a.m.call(b, 1);
```

```expect

```

### case: undefined, and the receiver as this

```js
f.call(undefined, a);
o.m.call(o, a, b);
```

```expect
1:1 js/no-useless-call
2:1 js/no-useless-call
```

```fixed
f(a);
o.m(a, b);
```
