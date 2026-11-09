# js/no-shorthand-property-overrides — cases

### case: padding after padding-left

```jsx
const el = <div style={{ paddingLeft: 10, padding: 20 }} />;
```

```expect
1:43 js/no-shorthand-property-overrides
```

### case: background

```jsx
const el = <div style={{ backgroundRepeat: 'no-repeat', background: 'url(a.png)' }} />;
```

```expect
1:57 js/no-shorthand-property-overrides
```

### case: the right order

```jsx
const el = <div style={{ padding: 20, paddingLeft: 10 }} />;
```

```expect
(none)
```

### case: a vendor prefix

```jsx
const el = <div style={{ WebkitTransitionDelay: '1s', WebkitTransition: 'all 1s' }} />;
```

```expect
1:55 js/no-shorthand-property-overrides
```

### case: an indirect reset

```jsx
const el = <div style={{ borderImageSource: 'none', border: '1px solid' }} />;
```

```expect
1:53 js/no-shorthand-property-overrides
```

### case: a dynamic value

```jsx
const el = <div style={{ paddingLeft: left, padding: 20 }} />;
```

```expect
(none)
```

### case: a spread

```jsx
const el = <div style={{ paddingLeft: 10, ...rest, padding: 20 }} />;
```

```expect
(none)
```

### case: a component

```jsx
const el = <Box style={{ paddingLeft: 10, padding: 20 }} />;
```

```expect
(none)
```

### case: unrelated properties

```jsx
const el = <div style={{ color: 'red', padding: 20, marginLeft: 5 }} />;
```

```expect
(none)
```

### case: a TypeScript assertion

```tsx
const el = <div style={{ paddingLeft: 10, padding: 20 } as const} />;
```

```expect
1:43 js/no-shorthand-property-overrides
```
