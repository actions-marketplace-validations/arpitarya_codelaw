# react/no-react-forward-ref — cases

### case: through React

```jsx
import React from "react";
export const Input = React.forwardRef((props, ref) => <input ref={ref} {...props} />);
```

```expect
2:22 react/no-react-forward-ref
```

### case: another forwardRef

```jsx
import { forwardRef } from "./mine";
export const Input = forwardRef(x);
```

```expect
(none)
```
