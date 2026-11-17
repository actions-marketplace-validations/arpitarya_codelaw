# react/no-unsafe-iframe-sandbox — cases

### case: scripts alone

```jsx
const f = <iframe sandbox="allow-scripts" src={u} />;
```

```expect
(none)
```

### case: both, in the other order

```jsx
const f = <iframe sandbox="allow-same-origin allow-forms allow-scripts" src={u} />;
```

```expect
1:11 react/no-unsafe-iframe-sandbox
```
