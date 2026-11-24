# css/no-self-nested-layer — cases

### case: a different inner name

```css
@layer a { @layer b { x { y: z } } }
```

```expect
(none)
```

### case: the same name two levels down

```css
@layer a { @layer b { @layer a { x { y: z } } } }
```

```expect
1:23 css/no-self-nested-layer
```
