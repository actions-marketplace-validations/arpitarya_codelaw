# js/no-late-current-target-access — cases

### case: in a timer callback

```js
el.onclick = (e) => setTimeout(() => e.currentTarget.blur());
```

```expect
1:38 js/no-late-current-target-access
```

### case: read inside the awaited expression

```js
el.onclick = async (e) => {
  await send(e.currentTarget.value);
};
```

```expect
(none)
```

### case: a parameter that is not an event

```js
async function f(widget) {
  await p;
  return widget.currentTarget;
}
```

```expect
(none)
```
