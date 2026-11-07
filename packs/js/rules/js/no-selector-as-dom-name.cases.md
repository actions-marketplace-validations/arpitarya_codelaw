# js/no-selector-as-dom-name — cases

### case: a class name

```js
menu.classList.add(".open");
```

```expect
1:1 js/no-selector-as-dom-name
```

```fixed
menu.classList.add("open");
```
