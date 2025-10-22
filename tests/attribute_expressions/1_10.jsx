import * as styles from "./styles.module.css";
import { binding } from "somewhere";

function refFn() {}
const refConst = null;

const selected = true;
let id = "my-h1";
let link;
const template = (
  <div id="main" {...results} classList={{ selected: unknown }} style={{ color }}>
    <h1
      class="base"
      id={id}
      {...results()}
      foo
      disabled
      title={welcoming()}
      style={{ "background-color": color(), "margin-right": "40px" }}
      classList={{ dynamic: dynamic(), selected }}
    >
      <a href={"/"} ref={link} classList={{ "ccc ddd": true }}>
        Welcome
      </a>
    </h1>
  </div>
);

const template2 = (
  <div {...getProps("test")}>
    <div textContent={rowId} />
    <div textContent={row.label} />
    <div innerHTML={"<div/>"} />
  </div>
);

const template3 = (
  <div
    foo
    id={/*@once*/ state.id}
    style={/*@once*/ { "background-color": state.color }}
    name={state.name}
    textContent={/*@once*/ state.content}
  />
);

const template4 = <div class="hi" className={state.class} classList={{ "ccc:ddd": true }} />;

const template5 = <div class="a" className="b"></div>;

const template6 = <div style={someStyle()} textContent="Hi" />;

let undefVar;
const template7 = (
  <div
    style={{ "background-color": color(), "margin-right": "40px", ...props.style }}
    style:padding-top={props.top}
    class:my-class={props.active}
    class:other-class={undefVar}
    classList={{ "other-class2": undefVar }}
  />
);

let refTarget;
const template8 = <div ref={refTarget} />;

const template9 = <div ref={e => console.log(e)} />;

const template10 = <div ref={refFactory()} />;
