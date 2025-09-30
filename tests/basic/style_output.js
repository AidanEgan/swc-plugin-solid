import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { setStyleProperty as _$setStyleProperty } from "solid-js/web";
import { style as _$style } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<button type=button>"), _tmpl$2 = /*#__PURE__*/ _$template("<button>");
import { render } from "solid-js/web";
import { createSignal } from "solid-js";
function Counter() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$0 = _tmpl$1();
        _$setStyleProperty(_el$0, "border", "1px solid red");
        _$setStyleProperty(_el$0, "padding", "2px");
        _el$0.$$click = increment;
        _$insert(_el$0, count);
        return _el$0;
    })();
}
function Counter2() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$1 = _tmpl$1();
        _$setStyleProperty(_el$1, "padding", "2px");
        _$style(_el$1, {
            [1]: "1px solid red"
        });
        _el$1.$$click = increment;
        _$insert(_el$1, count);
        return _el$1;
    })();
}
function Counter3() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$2 = _tmpl$1();
        _el$2.$$click = increment;
        _$insert(_el$2, count);
        _$effect(()=>_$setStyleProperty(_el$2, "border", props.border));
        return _el$2;
    })();
}
function Counter5() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$3 = _tmpl$2();
        _$insert(_el$3, count);
        _$effect(()=>_$style(_el$3, {
                border: "1px solid red",
                ...vals
            }));
        return _el$3;
    })();
}
function Counter4() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$4 = _tmpl$2();
        _$style(_el$4, {
            border: "1px solid red",
            ...vals
        });
        _$insert(_el$4, count);
        return _el$4;
    })();
}
render(()=>_$createComponent(Counter, {}), document.getElementById("app"));
_$delegateEvents([
    "click"
]);
