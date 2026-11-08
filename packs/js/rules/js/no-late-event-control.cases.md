# js/no-late-event-control — cases

### case: a generator handler

```js
el.onclick = function* (event) {
  event.stopPropagation();
};
```

```expect
2:3 js/no-late-event-control
```

### case: computed name

```js
el.onclick = (e) => {
  e["stopPropagation"]();
};
```

```expect
(none)
```
