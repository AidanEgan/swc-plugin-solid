import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { style as _$style } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { setStyleProperty as _$setStyleProperty } from "solid-js/web";
var _tmpl$ = /*#__PURE__*/_$template(`<button type=button>`);
import { render } from "solid-js/web";
import { createSignal } from "solid-js";
function Counter() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  return (() => {
    var _el$ = _tmpl$();
    _el$.$$click = increment;
    _$setStyleProperty(_el$, "border", "1px solid red");
    _$setStyleProperty(_el$, "padding", "2px");
    _$insert(_el$, count);
    return _el$;
  })();
}
function Counter2() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  return (() => {
    var _el$2 = _tmpl$();
    _el$2.$$click = increment;
    _$style(_el$2, {
      [1]: "1px solid red"
    });
    _$setStyleProperty(_el$2, "padding", "2px");
    _$insert(_el$2, count);
    return _el$2;
  })();
}
function Counter3() {
  const [count, setCount] = createSignal(1);
  const increment = () => setCount(count => count + 1);
  return (() => {
    var _el$3 = _tmpl$();
    _el$3.$$click = increment;
    _$insert(_el$3, count);
    _$effect(_$p => _$setStyleProperty(_el$3, "border", props.border));
    return _el$3;
  })();
}
render(() => _$createComponent(Counter, {}), document.getElementById("app"));
_$delegateEvents(["click"]);
