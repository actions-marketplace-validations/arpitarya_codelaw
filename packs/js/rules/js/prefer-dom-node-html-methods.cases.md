# js/prefer-dom-node-html-methods — cases

### case: a write stays

```js
panel.innerHTML = markup;
```

```expect
(none)
```

### case: an append stays

```js
panel.innerHTML += markup;
```

```expect
(none)
```
