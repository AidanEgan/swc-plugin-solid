import { template as _$template } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { setStyleProperty as _$setStyleProperty } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { style as _$style } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div id=main><h1 class=base id=my-h1><a href=/>Welcome"), _tmpl$2 = /*#__PURE__*/ _$template("<div><div></div> <div></div><div>"), _tmpl$3 = /*#__PURE__*/ _$template("<div foo>"), _tmpl$4 = /*#__PURE__*/ _$template("<div>"), _tmpl$5 = /*#__PURE__*/ _$template('<div class="a b">'), _tmpl$6 = /*#__PURE__*/ _$template("<div style=margin-right:40px>");
import * as styles from "./styles.module.css";
import { binding } from "somewhere";
function refFn() {}
const refConst = null;
const selected = true;
let id = "my-h1";
let link;
const template = (()=>{
    var _el$0 = _tmpl$1(), _el$1 = _el$0.firstChild, _el$2 = _el$1.firstChild;
    _$spread(_el$0, _$mergeProps(results, {
        classList: {
            selected: unknown
        },
        style: {
            color
        }
    }), false, true);
    _$spread(_el$1, _$mergeProps(results(), {
        foo: "",
        disabled: true,
        get title () {
            return welcoming();
        },
        get style () {
            return {
                "background-color": color(),
                "margin-right": "40px"
            };
        },
        get classList () {
            return {
                dynamic: dynamic(),
                selected
            };
        }
    }), false, true);
    var _ref$1 = link;
    typeof _ref$1 === "function" ? _$use(_ref$1, _el$2) : link = _el$2;
    _$classList(_el$2, {
        "ccc ddd": true
    });
    return _el$0;
})();
const template2 = (()=>{
    var _el$4 = _tmpl$2(), _el$5 = _el$4.firstChild, _el$6 = _el$5.nextSibling, _el$7 = _el$6.firstChild, _el$8 = _el$6.nextSibling;
    _$spread(_el$4, _$mergeProps(() => getProps("test")), false, true);
    _el$5.textContent = rowId;
    _el$8.innerHTML = "<div/>";
    _$effect(()=>_el$7.data = row.label);
    return _el$4;
})();
const template3 = (()=>{
    var _el$9 = _tmpl$3();
    _$setAttribute(_el$9, "id", state.id);
    _$setStyleProperty(_el$9, "background-color", state.color);
    _el$9.textContent = state.content;
    _$effect(()=>_$setAttribute(_el$9, "name", state.name));
    return _el$9;
})();
const template4 = (()=>{
    var _el$10 = _tmpl$4();
    _$classList(_el$10, {
        "ccc:ddd": true
    });
    _$effect(()=>_$className(_el$10, `hi ${state.class || ""}`));
    return _el$10;
})();
const template5 = _tmpl$5();
const template6 = (()=>{
    var _el$12 = _tmpl$4();
    _el$12.textContent = "Hi";
    _$effect((_p$)=>_$style(_el$12, someStyle(), _p$));
    return _el$12;
})();
let undefVar;
const template7 = (()=>{
    var _el$13 = _tmpl$6();
    _el$13.classList.toggle("other-class", !!undefVar);
    _el$13.classList.toggle("other-class2", !!undefVar);
    _$effect((_p$)=>{
        var _v$4 = {
            "background-color": color(),
            ...props.style
        }, _v$5 = props.top, _v$6 = !!props.active;
        _p$.a = _$style(_el$13, _v$4, _p$.a);
        _v$5 !== _p$.b && _$setStyleProperty(_el$13, "padding-top", _p$.b = _v$5);
        _v$6 !== _p$.c && _el$13.classList.toggle("my-class", _p$.c = _v$6);
        return _p$;
    }, {
        a: undefined,
        b: undefined,
        c: undefined
    });
    return _el$13;
})();
let refTarget;
const template8 = (()=>{
    var _el$14 = _tmpl$4();
    var _ref$2 = refTarget;
    typeof _ref$2 === "function" ? _$use(_ref$2, _el$14) : refTarget = _el$14;
    return _el$14;
})();
const template9 = (()=>{
    var _el$15 = _tmpl$4();
    _$use((e)=>console.log(e), _el$15);
    return _el$15;
})();
const template10 = (()=>{
    var _el$16 = _tmpl$4();
    var _ref$3 = refFactory();
    typeof _ref$3 === "function" && _$use(_ref$3, _el$16);
    return _el$16;
})();
