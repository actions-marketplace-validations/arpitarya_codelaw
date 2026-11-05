### case: the children prop is reported on a self-closing element

```jsx
const a = <div dangerouslySetInnerHTML={{ __html: h }} children={x} />;
```

```expect
1:11 react/no-danger-with-children
```

