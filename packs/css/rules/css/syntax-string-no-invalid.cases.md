# Cases: css/syntax-string-no-invalid

### case: a misspelt type takes the nearest name

```css
@property --x {
  syntax: "<colr>";
  inherits: false;
  initial-value: red;
}
```

```expect
2:11 css/syntax-string-no-invalid
```

```fixed
@property --x {
  syntax: "<color>";
  inherits: false;
  initial-value: red;
}
```

### case: a missing bracket is reported without a fix

```css
@property --x {
  syntax: "<length";
  inherits: false;
  initial-value: 0px;
}
```

```expect
2:11 css/syntax-string-no-invalid
```
