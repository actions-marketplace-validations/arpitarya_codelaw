### case: a write that reads the old state carries no edit

```jsx
class Counter extends React.Component {
  inc() {
    this.state.count = this.state.count + 1;
  }
}
```

```expect
3:5 react/no-direct-mutation-state
```
