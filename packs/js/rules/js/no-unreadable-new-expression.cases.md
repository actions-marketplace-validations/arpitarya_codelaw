# js/no-unreadable-new-expression — cases

### case: a property read

```js
const Bar = new Foo().Bar;
```

```expect
1:13 js/no-unreadable-new-expression
```

### case: a static member constructor stays

```js
const b = new Foo.Bar();
```

```expect
(none)
```

### case: a parenthesised instance stays

```js
const b = (new Foo()).bar;
```

```expect
(none)
```
