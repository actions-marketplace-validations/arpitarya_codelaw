# Cases: css/at-rule-prelude-no-invalid

### case: a font-face with a prelude

```css
@font-face foo {
  font-family: "A";
}
```

```expect
1:12 css/at-rule-prelude-no-invalid
```

```fixed
@font-face {
  font-family: "A";
}
```

### case: a keyframes named none is reported without a fix

```css
@keyframes none {
  from {
    opacity: 0;
  }
}
```

```expect
1:12 css/at-rule-prelude-no-invalid
```
