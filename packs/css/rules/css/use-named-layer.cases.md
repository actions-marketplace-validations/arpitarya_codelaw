# css/use-named-layer — cases

### case: an order statement

```css
@layer reset, base;
```

```expect
(none)
```

### case: anonymous

```css
@layer { a { color: red; } }
```

```expect
1:1 css/use-named-layer
```
