import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { setBoolAttribute as _$setBoolAttribute } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template('<div>"0"'), _tmpl$2 = /*#__PURE__*/ _$template("<div>undefined"), _tmpl$3 = /*#__PURE__*/ _$template("<div>null"), _tmpl$4 = /*#__PURE__*/ _$template("<div>boolTest()"), _tmpl$5 = /*#__PURE__*/ _$template("<div>boolTest"), _tmpl$6 = /*#__PURE__*/ _$template("<div>boolTestBinding"), _tmpl$7 = /*#__PURE__*/ _$template("<div>boolTestObjBinding.value"), _tmpl$8 = /*#__PURE__*/ _$template("<div>fn"), _tmpl$9 = /*#__PURE__*/ _$template("<div before quack>should have space before"), _tmpl$10 = /*#__PURE__*/ _$template("<div before quack after>should have space before/after");
const template51 = _tmpl$1();
const template52 = _tmpl$2();
const template53 = _tmpl$3();
const template54 = (()=>{
    var _el$6 = _tmpl$4();
    _$effect(()=>_$setBoolAttribute(_el$6, "quack", boolTest()));
    return _el$6;
})();
const template55 = (()=>{
    var _el$8 = _tmpl$5();
    _$setBoolAttribute(_el$8, "quack", boolTest);
    return _el$8;
})();
const template56 = (()=>{
    var _el$10 = _tmpl$6();
    _$setBoolAttribute(_el$10, "quack", boolTestBinding);
    return _el$10;
})();
const template57 = (()=>{
    var _el$12 = _tmpl$7();
    _$effect(()=>_$setBoolAttribute(_el$12, "quack", boolTestObjBinding.value));
    return _el$12;
})();
const template58 = (()=>{
    var _el$14 = _tmpl$8();
    _$setBoolAttribute(_el$14, "quack", ()=>false);
    return _el$14;
})();
const template59 = _tmpl$9();
const template60 = _tmpl$10();
