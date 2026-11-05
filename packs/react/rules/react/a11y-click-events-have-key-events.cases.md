
### case: a self-closing element inside a parent keeps the parent's closing tag
```jsx
const c = <p><img onClick={open} /></p>;
```
```expect
1:14 react/a11y-click-events-have-key-events
```
