# js/no-mutate-reactive-state-in-updated-hook — cases

### case: guarded

```js
import { ref, onUpdated } from "vue";
const n = ref(0);
onUpdated(() => {
  if (n.value > 9) return;
  n.value++;
});
```

```expect
(none)
```

### case: defineComponent and Date.now

```js
import { reactive, defineComponent } from "vue";
const s = reactive({ t: 0 });
defineComponent({
  updated() {
    s.t = Date.now();
  },
});
```

```expect
5:5 js/no-mutate-reactive-state-in-updated-hook
```
