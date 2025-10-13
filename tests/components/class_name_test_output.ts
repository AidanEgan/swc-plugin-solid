import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template('<button type=button class="bg-red p-2 m-4">Click Me!'), _tmpl$2 = /*#__PURE__*/ _$template('<button type=button class="bg-purple w-[10] h-4">Click Me!'), _tmpl$3 = /*#__PURE__*/ _$template('<button class="border-solid border-2 border-pink p-10">Click Me!'), _tmpl$4 = /*#__PURE__*/ _$template("<button>Click Me!"), _tmpl$5 = /*#__PURE__*/ _$template("<button>");
function Counter() {
    const staticClassName = "bg-red p-2 m-4";
    return _tmpl$1();
}
const Counter2 = ()=>{
    return _tmpl$2();
};
function Counter3() {
    return _tmpl$3();
}
function Counter4() {
    return _tmpl$3();
}
function Counter5() {
    return (()=>{
        var _el$8 = _tmpl$4();
        _$className(_el$8, classnameident);
        return _el$8;
    })();
}
function Counter6() {
    return (()=>{
        var _el$10 = _tmpl$4();
        _$effect(()=>_$className(_el$10, classnameeffect()));
        return _el$10;
    })();
}
function Counter7() {
    const [count, setCount] = createSignal(1);
    const increment = ()=>setCount((count)=>count + 1);
    return (()=>{
        var _el$12 = _tmpl$5();
        _$className(_el$12, getClass());
        _$insert(_el$12, count);
        return _el$12;
    })();
}
