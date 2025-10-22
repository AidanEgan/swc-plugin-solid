import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { style as _$style } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div>"), _tmpl$2 = /*#__PURE__*/ _$template('<div data="hi" data2=">'), _tmpl$3 = /*#__PURE__*/ _$template("<a>"), _tmpl$4 = /*#__PURE__*/ _$template("<div><a>"), _tmpl$5 = /*#__PURE__*/ _$template("<div start=Hi>Hi"), _tmpl$6 = /*#__PURE__*/ _$template("<label><span>Input is</span><input><div>"), _tmpl$7 = /*#__PURE__*/ _$template('<div style="color: red;\n    background-color: blue !important;\n    border: 1px solid black;\n    font-size: 12px;" random="random1 random2\n    random3 random4" class="class1 class2\n    class3 class4\n    class5 class6">');
const template21 = (()=>{
    var _el$0 = _tmpl$1();
    _$effect((_p$)=>_$style(_el$0, {
            a: "static",
            ...rest
        }, _p$));
    return _el$0;
})();
const template22 = _tmpl$2();
const template23 = (()=>{
    var _el$2 = _tmpl$1();
    _$insert(_el$2, "t" in test && "true");
    _$effect(()=>_el$2.disabled = "t" in test);
    return _el$2;
})();
const template24 = (()=>{
    var _el$3 = _tmpl$3();
    _$spread(_el$3, _$mergeProps(props, {
        something: ""
    }), false, false);
    return _el$3;
})();
const template25 = (()=>{
    var _el$4 = _tmpl$4(), _el$5 = _el$4.firstChild;
    _$spread(_el$5, _$mergeProps(props, {
        something: ""
    }), false, false);
    _$insert(_el$4, ()=>props.children, _el$5);
    return _el$4;
})();
const template26 = (()=>{
    var _el$6 = _tmpl$5();
    _$setAttribute(_el$6, "middle", middle);
    _$spread(_el$6, spread, false, true);
    return _el$6;
})();
const template27 = (()=>{
    var _el$8 = _tmpl$5();
    _$spread(_el$8, _$mergeProps(first, {
        middle: middle
    }, second), false, true);
    return _el$8;
})();
const template28 = (()=>{
    var _el$10 = _tmpl$6(), _el$11 = _el$10.firstChild, _el$12 = _el$11.firstChild, _el$13 = _el$11.nextSibling, _el$14 = _el$13.nextSibling;
    _$spread(_el$10, _$mergeProps(api), false, true);
    _$spread(_el$11, _$mergeProps(api), false, true);
    _$insert(_el$11, api() ? "checked" : "unchecked", null);
    _$spread(_el$13, _$mergeProps(api), false, false);
    _$spread(_el$14, _$mergeProps(api), false, false);
    return _el$10;
})();
const template29 = (()=>{
    var _el$15 = _tmpl$1();
    _$setAttribute(_el$15, "attribute", !!someValue);
    _$insert(_el$15, !!someValue);
    return _el$15;
})();
const template30 = _tmpl$7();
