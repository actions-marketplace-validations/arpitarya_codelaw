# tanstack-query/prefer-query-options — cases

### case: a query client method with an inline key

```ts
import { useQueryClient } from '@tanstack/react-query';

function useRead() {
  const queryClient = useQueryClient();
  return queryClient.getQueryData(['todos']);
}
```

```expect
5:35 tanstack-query/prefer-query-options
```

### case: a filter with an inline key

```ts
import { QueryClient } from '@tanstack/react-query';

const client = new QueryClient();
client.invalidateQueries({ queryKey: ['todos'] });
```

```expect
4:26 tanstack-query/prefer-query-options
```

### case: a key from a key factory is fine

```ts
import { QueryClient } from '@tanstack/react-query';

const client = new QueryClient();
client.invalidateQueries({ queryKey: todoKeys.all });
```

```expect
(none)
```

### case: useQueries with a map callback

```ts
import { useQueries } from '@tanstack/react-query';

function useAll(ids: number[]) {
  return useQueries({ queries: ids.map((id) => ({ queryKey: ['todo', id], queryFn: () => load(id) })) });
}
```

```expect
4:49 tanstack-query/prefer-query-options
```

### case: options built with queryOptions

```ts
import { queryOptions, useQuery } from '@tanstack/react-query';

function useTodos() {
  return useQuery(queryOptions({ queryKey: ['todos'], queryFn: fetchTodos }));
}
```

```expect
(none)
```

### case: not a TanStack Query import

```ts
import { useQuery } from './my-query';

function useTodos() {
  return useQuery({ queryKey: ['todos'] });
}
```

```expect
(none)
```
