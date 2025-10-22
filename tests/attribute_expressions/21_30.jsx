const template21 = <div style={{ a: "static", ...rest }}></div>;

const template22 = <div data='"hi"' data2={'"'} />;

const template23 = <div disabled={"t" in test}>{"t" in test && "true"}</div>;

const template24 = <a {...props} something />;

const template25 = (
  <div>
    {props.children}
    <a {...props} something />
  </div>
);

const template26 = (
  <div start="Hi" middle={middle} {...spread}>
    Hi
  </div>
);

const template27 = (
  <div start="Hi" {...first} middle={middle} {...second}>
    Hi
  </div>
);

const template28 = (
  <label {...api()}>
    <span {...api()}>Input is {api() ? "checked" : "unchecked"}</span>
    <input {...api()} />
    <div {...api()} />
  </label>
);

const template29 = <div attribute={!!someValue}>{!!someValue}</div>;

const template30 = (
  <div
    class="class1 class2
    class3 class4
    class5 class6"
    style="color: red;
    background-color: blue !important;
    border: 1px solid black;
    font-size: 12px;"
    random="random1 random2
    random3 random4"
  />
);
