---
id: redux-toolkit/no-side-effects-in-reducer
title: "A reducer has no side effects"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a case reducer whose side effect sits in an object built from `action.payload` gets a `prepare` callback that builds the object, and reads it from the payload; the action creator's argument stays the same, the effect now runs when the action is made. Any other effect is manual"
fix_exception: restructure
stability: preview
since: 0.1.0
---

A reducer — a `createSlice` case reducer, a builder's `addCase`/`addMatcher`/`addDefaultCase`
callback, or a hand-written `(state, action)` reducer — doesn't fetch, wait, start timers, read the
clock or a random source, touch storage, or dispatch.

**Why:** A reducer must give the same next state for the same state and action, so that replaying actions
reproduces the state. *Style Guide, Priority A: Reducers Must Not Have Side Effects.*

```codelaw
native: redux-toolkit/no-side-effects-in-reducer
```

## How to fix

1. Make random IDs and timestamps in the action's `prepare` callback, so they arrive in the payload.
2. Move fetching, timers and storage into a thunk (`createAsyncThunk`) or a listener.

## Examples

### JavaScript

#### Don't

```js
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded(state, action) {
      state.push({ id: nanoid(), text: action.payload });
    },
  },
});
```

#### Fixed

```js
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(payload) {
        return { payload: { id: nanoid(), text: payload } };
      },
    },
  },
});
```

#### Do

```js
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(text) {
        return { payload: { id: nanoid(), text } };
      },
    },
  },
});
```

### JSX

#### Don't

```jsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded(state, action) {
      state.push({ id: nanoid(), text: action.payload });
    },
  },
});
```

#### Fixed

```jsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(payload) {
        return { payload: { id: nanoid(), text: payload } };
      },
    },
  },
});
```

#### Do

```jsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(text) {
        return { payload: { id: nanoid(), text } };
      },
    },
  },
});
```

### TypeScript

#### Don't

```ts
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded(state, action) {
      state.push({ id: nanoid(), text: action.payload });
    },
  },
});
```

#### Fixed

```ts
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(payload) {
        return { payload: { id: nanoid(), text: payload } };
      },
    },
  },
});
```

#### Do

```ts
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(text) {
        return { payload: { id: nanoid(), text } };
      },
    },
  },
});
```

### TSX

#### Don't

```tsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded(state, action) {
      state.push({ id: nanoid(), text: action.payload });
    },
  },
});
```

#### Fixed

```tsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(payload) {
        return { payload: { id: nanoid(), text: payload } };
      },
    },
  },
});
```

#### Do

```tsx
const todosSlice = createSlice({
  name: 'todos',
  initialState: [],
  reducers: {
    todoAdded: {
      reducer(state, action) {
        state.push(action.payload);
      },
      prepare(text) {
        return { payload: { id: nanoid(), text } };
      },
    },
  },
});
```
