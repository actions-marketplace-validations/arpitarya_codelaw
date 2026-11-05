# js/no-void — cases

### case: a literal and a statement

```js
const a = void 0;
void save();
```

```expect
1:11 js/no-void
2:1 js/no-void
```

```fixed
const a = undefined;
save();
```

### case: a value position with a call has no edit

```js
const b = void save();
```

```expect
1:11 js/no-void
```
