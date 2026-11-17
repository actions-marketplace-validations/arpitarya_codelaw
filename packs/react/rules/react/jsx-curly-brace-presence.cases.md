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

### case: a child string

```jsx
const b = <b>{"Save"}</b>;
```

```expect
1:14 react/jsx-curly-brace-presence
```

```fixed
const b = <b>Save</b>;
```

### case: a single-quoted child between elements

```jsx
const b = <p><i /> {'and'} <i /></p>;
```

```expect
1:20 react/jsx-curly-brace-presence
```

```fixed
const b = <p><i /> and <i /></p>;
```

### case: a child that text would read as markup

```jsx
const a = <b>{"a < b"}</b>;
const c = <b>{"R&D"}</b>;
const d = <b>{"a{b}"}</b>;
```

```expect
```

### case: a child that is a space, empty or padded

```jsx
const a = <b>{" "}x</b>;
const c = <b>{""}</b>;
const d = <b>{" x"}</b>;
```

```expect
```

### case: a child string with an escape

```jsx
const b = <b>{"a\nb"}</b>;
```

```expect
```

### case: a child joined to text on the next line

```jsx
const b = (
  <p>
    text
    {"more"}
  </p>
);
```

```expect
```

### case: a child alone on its line

```jsx
const b = (
  <p>
    {"more"}
  </p>
);
```

```expect
3:5 react/jsx-curly-brace-presence
```

```fixed
const b = (
  <p>
    more
  </p>
);
```

### case: two braced children on separate lines

```jsx
const b = (
  <p>
    {"a"}
    {"b"}
  </p>
);
```

```expect
```

### case: a child that is not a string

```jsx
const b = <b>{name}</b>;
```

```expect
```

### case: a character reference beside a child

```jsx
const b = <p>&amp;{"x"}</p>;
```

```expect
```
