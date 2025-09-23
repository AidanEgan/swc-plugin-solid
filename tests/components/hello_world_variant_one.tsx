import { render } from "solid-js/web";
import { createSignal } from "solid-js";

function Counter() {
  const [count, setCount] = createSignal(1);

  return (
    <button type="button" onClick={increment}>
      {count(1, 2)}
    </button>
  );
}

render(() => <Counter />, document.getElementById("app")!);
