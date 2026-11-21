# css/use-layered-styles — cases

### case: inside a media query that is inside a layer

```css
@layer base { @media print { a { color: red } } }
```

```expect
(none)
```

### case: inside a media query alone

```css
@media print { a { color: red } }
```

```expect
1:16 css/use-layered-styles
```

### case: nested rule

```css
@layer base { a { b { color: red } } }
```

```expect
(none)
```
