---
id: tanstack-query/prefer-query-options
title: "Query options and keys come from `queryOptions()`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the options move into a named `queryOptions()` call whose name and place only the author can choose"
fix_exception: restructure
stability: preview
ported_from: ["@tanstack/eslint-plugin-query/prefer-query-options"]
since: 0.5.0
---

A query is not described by an object written at the call. `useQuery`, `useInfiniteQuery`, `useSuspenseQuery`, `useSuspenseInfiniteQuery`, `usePrefetchQuery`, `usePrefetchInfiniteQuery`, the entries of `useQueries({ queries })` and `useSuspenseQueries`, and a QueryClient's `fetchQuery`, `prefetchQuery`, `fetchInfiniteQuery`, `prefetchInfiniteQuery`, `ensureQueryData` and `ensureInfiniteQueryData` are given the result of `queryOptions()` or `infiniteQueryOptions()`, not an object literal with `queryKey` or `queryFn`. A query key typed at a QueryClient method (`getQueryData(['todos'])`, `setQueryData`, `getQueryState`, `setQueryDefaults`, `getQueryDefaults`) or in a filter (`invalidateQueries({ queryKey: ['todos'] })`, `useIsFetching`, …) is taken from the same result.

**Why:** A key written at each call site drifts from the key the query is fetched with, so an invalidation or a cache read silently misses. `queryOptions()` holds the key and the function in one place and types the data that comes back.

```codelaw
native: tanstack-query/prefer-query-options
```

## How to fix

1. Move the `queryKey` and `queryFn` into a function that returns `queryOptions({ … })`.
2. Pass its result to the hook or the client method, and read the key as `todoOptions().queryKey` where a key is needed.


## Examples

### JavaScript

#### Don't

```js
import { useQuery } from '@tanstack/react-query';

function useTodos() {
  return useQuery({ queryKey: ['todos'], queryFn: fetchTodos });
}
```

#### Do

```js
import { queryOptions, useQuery } from '@tanstack/react-query';

const todosOptions = () => queryOptions({ queryKey: ['todos'], queryFn: fetchTodos });

function useTodos() {
  return useQuery(todosOptions());
}
```

### JSX

#### Don't

```jsx
import { useQuery } from '@tanstack/react-query';

function useTodos() {
  return useQuery({ queryKey: ['todos'], queryFn: fetchTodos });
}
```

#### Do

```jsx
import { queryOptions, useQuery } from '@tanstack/react-query';

const todosOptions = () => queryOptions({ queryKey: ['todos'], queryFn: fetchTodos });

function useTodos() {
  return useQuery(todosOptions());
}
```

### TypeScript

#### Don't

```ts
import { useQuery } from '@tanstack/react-query';

function useTodos() {
  return useQuery({ queryKey: ['todos'], queryFn: fetchTodos });
}
```

#### Do

```ts
import { queryOptions, useQuery } from '@tanstack/react-query';

const todosOptions = () => queryOptions({ queryKey: ['todos'], queryFn: fetchTodos });

function useTodos() {
  return useQuery(todosOptions());
}
```

### TSX

#### Don't

```tsx
import { useQuery } from '@tanstack/react-query';

function useTodos() {
  return useQuery({ queryKey: ['todos'], queryFn: fetchTodos });
}
```

#### Do

```tsx
import { queryOptions, useQuery } from '@tanstack/react-query';

const todosOptions = () => queryOptions({ queryKey: ['todos'], queryFn: fetchTodos });

function useTodos() {
  return useQuery(todosOptions());
}
```

## Differences from the source

A QueryClient is recognised as upstream recognises it: a binding initialised by `useQueryClient()` or `new QueryClient()`, followed through `const` bindings and type assertions. A client that arrives as a parameter or a property is not found (codelaw has no type checker). Opt-in: upstream puts the rule in its `strict` preset only.
