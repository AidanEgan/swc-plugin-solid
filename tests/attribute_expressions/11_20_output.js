import { template as _$template } from "solid-js/web";
import { addEventListener as _$addEventListener } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div>"), _tmpl$2 = /*#__PURE__*/ _$template("<div onclick=console.log('hi')>"), _tmpl$3 = /*#__PURE__*/ _$template("<input type=checkbox>"), _tmpl$4 = /*#__PURE__*/ _$template("<div class=`a>`$`"), _tmpl$5 = /*#__PURE__*/ _$template('<button type=button class="static hi">Write'), _tmpl$6 = /*#__PURE__*/ _$template('<button class="a b c">Hi'), _tmpl$7 = /*#__PURE__*/ _$template('<div class="bg-red-500 flex flex-col">'), _tmpl$8 = /*#__PURE__*/ _$template("<div><input readOnly><input>");
const template11 = (()=>{
    var _el$0 = _tmpl$1();
    _$use(something, _el$0, ()=>true);
    _$use(another, _el$0, ()=>thing);
    _$use(zero, _el$0, ()=>0);
    return _el$0;
})();
const template12 = (()=>{
    var _el$1 = _tmpl$2();
    _el$1.htmlFor = thing;
    _el$1.number = 123;
    return _el$1;
})();
const template13 = (()=>{
    var _el$2 = _tmpl$3();
    _el$2.checked = true;
    return _el$2;
})();
const template14 = (()=>{
    var _el$3 = _tmpl$3();
    _$effect(()=>_el$3.checked = state.visible);
    return _el$3;
})();
const template15 = _tmpl$4();
const template16 = _tmpl$5();
const template17 = (()=>{
    var _el$8 = _tmpl$6();
    _$addEventListener(_el$8, "click", increment, true);
    return _el$8;
})();
const template18 = (()=>{
    var _el$10 = _tmpl$1();
    _$spread(_el$10, _$mergeProps({
        get [key()] () {
            return props.value;
        }
    }), false, false);
    return _el$10;
})();
const template19 = _tmpl$7();
const template20 = (()=>{
    var _el$12 = _tmpl$8(), _el$13 = _el$12.firstChild, _el$14 = _el$13.nextSibling;
    _$addEventListener(_el$13, "input", doSomething, true);
    _$addEventListener(_el$14, "input", doSomethingElse, true);
    _el$14.readOnly = value;
    _$effect((_p$)=>{
        var _v$0 = min(), _v$1 = max(), _v$2 = min(), _v$3 = max();
        _v$0 !== _p$.a && _$setAttribute(_el$13, "min", _p$.a = _v$0);
        _v$1 !== _p$.b && _$setAttribute(_el$13, "max", _p$.b = _v$1);
        _v$2 !== _p$.c && _$setAttribute(_el$14, "min", _p$.c = _v$2);
        _v$3 !== _p$.d && _$setAttribute(_el$14, "max", _p$.d = _v$3);
        return _p$;
    }, {
        a: undefined,
        b: undefined,
        c: undefined,
        d: undefined
    });
    _$effect(()=>_el$14.checked = s2());
    _$effect(()=>_el$13.value = s());
    return _el$12;
})();
_$delegateEvents([
    "click",
    "input"
]);
