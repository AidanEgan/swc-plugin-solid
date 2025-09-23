import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template('<button class="bg-red p-2 m-4" type=button>Click Me!'), _tmpl$2 = /*#__PURE__*/ _$template('<button class="bg-purple w-[10] h-4" type=button>Click Me!'), _tmpl$3 = /*#__PURE__*/ _$template('<button class="border-solid border-2 border-pink p-10">Click Me!'), _tmpl$4 = /*#__PURE__*/ _$template("<button>Click Me!");
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
        var _el$8 = _tmpl$4(), _el$9 = _el$8.nextSibling;
        _$className(_el$8, classnameident);
        return _el$8;
    })();
}
function Counter6() {
    return (()=>{
        var _el$10 = _tmpl$4(), _el$11 = _el$10.nextSibling;
        _$effect(()=>_$className(_el$10, classnameeffect()));
        return _el$10;
    })();
}
