# Cases: css/named-grid-areas-no-invalid

### case: padding would leave an area that is not a rectangle

```css
a {
  grid-template-areas:
    "head side"
    "main head head";
}
```

```expect
3:5 css/named-grid-areas-no-invalid
```

### case: grid-template rows with track sizes

```css
a {
  grid-template:
    "head head" 40px
    "main" 1fr / 1fr 1fr;
}
```

```expect
3:5 css/named-grid-areas-no-invalid
```

```fixed
a {
  grid-template:
    "head head" 40px
    "main ." 1fr / 1fr 1fr;
}
```
