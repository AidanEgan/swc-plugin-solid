function Counter() {
  const staticClassName = "bg-red p-2 m-4";
  return (
    <button class={staticClassName} type="button">
      Click Me!
    </button>
  );
}

const Counter2 = () => {
  return (
    <button class="bg-purple w-[10] h-4" type="button">
      Click Me!
    </button>
  );
}

function Counter3() {
  return (
    <button class={"border-solid border-2 border-pink p-10"}>
      Click Me!
    </button>
  );
}

function Counter4() {
  return (
    <button className={"border-solid border-2 border-pink p-10"}>
      Click Me!
    </button>
  );
}

function Counter5() {
  return (
    <button class={classnameident}>
      Click Me!
    </button>
  );
}

function Counter6() {
  return (
    <button class={classnameeffect()}>
      Click Me!
    </button>
  );
}
