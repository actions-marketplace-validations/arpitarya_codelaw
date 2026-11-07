# js/no-invalid-style-set-property — cases

### case: important in the value

```js
element.style.setProperty("color", "red !important");
```

```expect
1:1 js/no-invalid-style-set-property
```

```fixed
element.style.setProperty("color", "red", "important");
```

### case: a priority with its bang

```js
element.style.setProperty("color", "red", "!important");
```

```expect
1:1 js/no-invalid-style-set-property
```

```fixed
element.style.setProperty("color", "red", "important");
```

### case: a custom property stays

```js
element.style.setProperty("--brandColor", "red");
```

```expect
(none)
```

### case: a correct call stays

```js
element.style.setProperty("color", "red", "important");
```

```expect
(none)
```
