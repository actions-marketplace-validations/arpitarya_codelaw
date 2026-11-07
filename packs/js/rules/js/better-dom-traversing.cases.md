# js/better-dom-traversing — cases

### case: childNodes

```js
const node = element.childNodes[0];
```

```expect
1:14 js/better-dom-traversing
```

```fixed
const node = element.firstChild;
```

### case: props.children stays

```js
const first = props.children[0];
```

```expect
(none)
```

### case: another index stays

```js
const second = list.children[1];
```

```expect
(none)
```
