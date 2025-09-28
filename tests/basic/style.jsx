import { render } from "solid-js/web";
import { createSignal } from "solid-js";

function Counter() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);

  return (
    <button style={{ border: "1px solid red", padding: "2px" }} type="button" onClick={increment}>
      {count()}
    </button>
  );
}

function Counter2() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);

  return (
    <button style={{ [1]: "1px solid red", padding: "2px" }} type="button" onClick={increment}>
      {count()}
    </button>
  );
}

function Counter3() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);

  return (
    <button style={{ border: props.border }} type="button" onClick={increment}>
      {count()}
    </button>
  );
}

render(() => <Counter />, document.getElementById("app"));
