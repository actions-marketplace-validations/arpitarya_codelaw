# js/table-header-reference — cases

### case: the right headers

```jsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="name">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```

```expect
(none)
```

### case: a header of another column

```jsx
const t = (
  <table>
    <thead>
      <tr><th id="name">Name</th><th id="age">Age</th></tr>
    </thead>
    <tbody>
      <tr><td headers="age">Ann</td><td headers="age">3</td></tr>
    </tbody>
  </table>
);
```

```expect
7:11 js/table-header-reference
```

### case: an id that is no header

```jsx
const t = (
  <table>
    <tr><th id="a">A</th></tr>
    <tr><td headers="nope">x</td></tr>
  </table>
);
```

```expect
4:9 js/table-header-reference
```

### case: a colSpan header covers both columns

```jsx
const t = (
  <table>
    <tr><th id="a" colSpan={2}>A</th></tr>
    <tr><td headers="a">1</td><td headers="a">2</td></tr>
  </table>
);
```

```expect
(none)
```

### case: a row header

```jsx
const t = (
  <table>
    <tr><th id="r">R</th><td headers="r">1</td></tr>
  </table>
);
```

```expect
(none)
```

### case: a component row is not known

```jsx
const t = (
  <table>
    <Row />
    <tr><td headers="nope">x</td></tr>
  </table>
);
```

```expect
(none)
```

### case: a cell without headers

```jsx
const t = (
  <table>
    <tr><td>x</td></tr>
  </table>
);
```

```expect
(none)
```
