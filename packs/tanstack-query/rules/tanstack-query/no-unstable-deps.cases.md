# tanstack-query/no-unstable-deps — cases

### case: a result passed on whole is not destructured

```js
import { useMutation } from '@tanstack/react-query';
import { useEffect } from 'react';

function useSave(log) {
  const mutation = useMutation({ mutationFn: save });
  useEffect(() => log(mutation), [mutation]);
}
```

```expect
6:35 tanstack-query/no-unstable-deps
```

### case: two fields, both listed

```js
import { useMutation } from '@tanstack/react-query';
import { useCallback } from 'react';

function useSave() {
  const mutation = useMutation({ mutationFn: save });
  return useCallback(() => mutation.isPending || mutation.mutate(), [mutation]);
}
```

```expect
6:70 tanstack-query/no-unstable-deps
```

```fixed
import { useMutation } from '@tanstack/react-query';
import { useCallback } from 'react';

function useSave() {
  const { isPending, mutate } = useMutation({ mutationFn: save });
  return useCallback(() => isPending || mutate(), [isPending, mutate]);
}
```
