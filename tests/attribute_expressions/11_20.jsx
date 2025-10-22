const template11 = <div use:something use:another={thing} use:zero={0} />;

const template12 = <div prop:htmlFor={thing} prop:number={123} attr:onclick="console.log('hi')" />;

const template13 = <input type="checkbox" checked={true} />;

const template14 = <input type="checkbox" checked={state.visible} />;

const template15 = <div class="`a">`$`</div>;

const template16 = (
  <button
    class="static"
    classList={{
      hi: "k"
    }}
    type="button"
  >
    Write
  </button>
);

const template17 = (
  <button
    classList={{
      a: true,
      b: true,
      c: true
    }}
    onClick={increment}
  >
    Hi
  </button>
);

const template18 = (
  <div
    {...{
      get [key()]() {
        return props.value;
      }
    }}
  />
);

const template19 = <div classList={{ "bg-red-500": true }} class="flex flex-col" />;

const template20 = (
  <div>
    <input value={s()} min={min()} max={max()} onInput={doSomething} readonly="" />
    <input checked={s2()} min={min()} max={max()} onInput={doSomethingElse} readonly={value} />
  </div>
);
