# react/jsx-curly-brace-presence — cases

### case: a string with an escape

```jsx
const b = <Button title={"a\nb"} />;
```

```expect
```

### case: a single-quoted string

```jsx
const b = <Button title={'Save'} />;
```

```expect
1:19 react/jsx-curly-brace-presence
```

```fixed
const b = <Button title='Save' />;
```
