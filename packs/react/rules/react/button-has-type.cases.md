# react/button-has-type — cases

### case: a spread may carry the type

```jsx
const b = <button {...props}>x</button>;
```

```expect
```

### case: self-closing

```jsx
const b = <button />;
```

```expect
1:11 react/button-has-type
```

```fixed
const b = <button type="button" />;
```

### case: a misspelt type

```jsx
const b = <button type="buton" onClick={save}>x</button>;
```

```expect
1:11 react/button-has-type
```

```fixed
const b = <button type="button" onClick={save}>x</button>;
```

### case: a misspelt type in braces

```jsx
const b = <button type={'sumbit'}>x</button>;
```

```expect
1:11 react/button-has-type
```

```fixed
const b = <button type={"button"}>x</button>;
```

### case: the three valid types

```jsx
const a = <button type="submit">x</button>;
const b = <button type={"reset"}>x</button>;
const c = <button type='button'>x</button>;
```

```expect
```

### case: a type held in an expression

```jsx
const b = <button type={kind}>x</button>;
```

```expect
```
