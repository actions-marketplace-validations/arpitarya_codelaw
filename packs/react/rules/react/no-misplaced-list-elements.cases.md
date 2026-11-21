# react/no-misplaced-list-elements — cases

### case: inside a component

```jsx
const l = <List><li>One</li></List>;
```

```expect
(none)
```

### case: inside a section

```jsx
const l = <section>
  <li>One</li>
</section>;
```

```expect
2:3 react/no-misplaced-list-elements
```
