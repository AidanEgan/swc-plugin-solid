import { render } from "solid-js/web";
import { createSignal } from "solid-js";

function Counter() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);

  return (
    <button type="button" classList={{...prop}}>
      {count()}
    </button>
  );
}

function Counter2() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);

  return (
    <button type="button"classList={{ bgred: true, padding: val }}>
      {count()}
    </button>
  );
}

function Counter3() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  const props = {
    key: value()
  };

  return (
    <button type="button" classList={{ "p-2 bg-red": true, [props.class]: true }}>
      {count()}
    </button>
  );
}

function Counter4() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  const props = {
    key: value()
  };

  return (
    <button type="button" classList={{ background: false, border: true }}>
      {count()}
    </button>
  );
}

function Counter5() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  const props = {
    key: value()
  };

  return (
    <button type="button" classList={{ padding: props.padding }}>
      {count()}
    </button>
  );
}

function Counter6() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  const props = {
    key: value()
  };

  return (
    <button type="button" classList={  /*@once*/  { padding: props.padding }}>
      {count()}
    </button>
  );
}

render(() => <Counter />, document.getElementById("app")!);
