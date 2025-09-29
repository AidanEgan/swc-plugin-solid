import { template as _$template } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<button type=button>"), _tmpl$2 = /*#__PURE__*/ _$template("<button type=button class=bgred>"), _tmpl$3 = /*#__PURE__*/ _$template("<button type=button class=border>");
import { render } from "solid-js/web";
import { createSignal } from "solid-js";
function Counter() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$0 = _tmpl$1();
        _$classList(_el$0, {
            ...prop
        });
        _$insert(_el$0, count);
        return _el$0;
    })();
}
function Counter2() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$1 = _tmpl$2();
        _el$1.classList.toggle("padding", !!val);
        _$insert(_el$1, count);
        return _el$1;
    })();
}
function Counter3() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    const props = {
        key: value()
    };
    return (()=>{
        var _el$2 = _tmpl$1();
        _$insert(_el$2, count);
        _$effect((_p$)=>_$classList(_el$2, {
                "p-2 bg-red": true,
                [props.class]: true
            }, _p$));
        return _el$2;
    })();
}
function Counter4() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    const props = {
        key: value()
    };
    return (()=>{
        var _el$3 = _tmpl$3();
        _$insert(_el$3, count);
        return _el$3;
    })();
}
function Counter5() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    const props = {
        key: value()
    };
    return (()=>{
        var _el$4 = _tmpl$1();
        _$insert(_el$4, count);
        _$effect(()=>_el$4.classList.toggle("padding", !!props.padding));
        return _el$4;
    })();
}
render(()=>_$createComponent(Counter, {}), document.getElementById("app")!);
