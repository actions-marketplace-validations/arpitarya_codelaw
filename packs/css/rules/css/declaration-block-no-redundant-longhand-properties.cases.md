# Cases: css/declaration-block-no-redundant-longhand-properties

### case: place-items from its two longhands

```css
a {
  align-items: center;
  justify-items: start;
}
```

```expect
2:3 css/declaration-block-no-redundant-longhand-properties
```

```fixed
a {
  place-items: center start;
}
```

### case: all !important stays !important

```css
a {
  overflow-x: hidden !important;
  color: red;
  overflow-y: auto !important;
}
```

```expect
2:3 css/declaration-block-no-redundant-longhand-properties
```

```fixed
a {
  color: red;
  overflow: hidden auto !important;
}
```

### case: a var() longhand is reported without a fix

```css
a {
  padding-top: var(--x);
  padding-right: 0;
  padding-bottom: 0;
  padding-left: 0;
}
```

```expect
2:3 css/declaration-block-no-redundant-longhand-properties
```

### case: an overlapping logical longhand between them is reported without a fix

```css
a {
  margin-top: 0;
  margin-right: 0;
  margin-inline-start: 1px;
  margin-bottom: 0;
  margin-left: 0;
}
```

```expect
2:3 css/declaration-block-no-redundant-longhand-properties
```

### case: font is not a box-model shorthand

```css
a {
  font-style: italic;
  font-variant: normal;
  font-weight: 700;
  font-stretch: normal;
  font-size: 1rem;
  line-height: 1.5;
  font-family: serif;
}
```

```expect
2:3 css/declaration-block-no-redundant-longhand-properties
```
