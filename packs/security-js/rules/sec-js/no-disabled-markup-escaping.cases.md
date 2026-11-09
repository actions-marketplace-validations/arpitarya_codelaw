# sec-js/no-disabled-markup-escaping — cases

### case: turned off

```js
engine.escapeMarkup = false;
```

```expect
1:1 sec-js/no-disabled-markup-escaping
```

### case: on a nested object

```js
app.view.escapeMarkup = false;
```

```expect
1:1 sec-js/no-disabled-markup-escaping
```

### case: left on

```js
engine.escapeMarkup = true;
```

```expect
(none)
```

### case: another property

```js
engine.escape = false;
```

```expect
(none)
```

### case: a function replaces it

```js
engine.escapeMarkup = (s) => clean(s);
```

```expect
(none)
```
