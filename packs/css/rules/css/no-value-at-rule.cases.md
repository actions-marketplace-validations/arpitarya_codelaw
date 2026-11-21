# css/no-value-at-rule — cases

### case: an import of values

```css
@value primary from "./colors.css";
```

```expect
1:1 css/no-value-at-rule
```

### case: another at-rule

```css
@media print { a { color: red } }
```

```expect
(none)
```
