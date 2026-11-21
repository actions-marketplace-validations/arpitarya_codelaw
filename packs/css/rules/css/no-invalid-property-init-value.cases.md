# css/no-invalid-property-init-value — cases

### case: a keyword alternative

```css
@property --a { syntax: "<color> | auto"; inherits: false; initial-value: auto; }
```

```expect
(none)
```

### case: a number where a length is needed

```css
@property --a { syntax: "<length>"; inherits: false; initial-value: 5; }
```

```expect
1:54 css/no-invalid-property-init-value
```
