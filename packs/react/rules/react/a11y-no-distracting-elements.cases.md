# react/a11y-no-distracting-elements — cases

### case: a blink element becomes a span

```jsx
const b = <blink>New</blink>;
```

```expect
1:11 react/a11y-no-distracting-elements
```

```fixed
const b = <span>New</span>;
```

### case: a self-closing marquee

```jsx
const m = <marquee />;
```

```expect
1:11 react/a11y-no-distracting-elements
```

```fixed
const m = <div />;
```
