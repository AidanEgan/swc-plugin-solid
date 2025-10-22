import { template as _$template } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div title=<u>data</u>>"), _tmpl$2 = /*#__PURE__*/ _$template("<div>"), _tmpl$3 = /*#__PURE__*/ _$template("<div truestr=true truestrjs=true>"), _tmpl$4 = /*#__PURE__*/ _$template("<div falsestr=false falsestrjs=false>");
import { binding } from "somewhere";
const refConst = null;
const template71 = _tmpl$1();
const template72 = (()=>{
    var _el$1 = _tmpl$2();
    _$use(binding, _el$1);
    return _el$1;
})();
const template73 = (()=>{
    var _el$2 = _tmpl$2();
    var _ref$1 = binding.prop;
    typeof _ref$1 === "function" ? _$use(_ref$1, _el$2) : binding.prop = _el$2;
    return _el$2;
})();
const template74 = (()=>{
    var _el$3 = _tmpl$2();
    var _ref$2 = refFn;
    typeof _ref$2 === "function" ? _$use(_ref$2, _el$3) : refFn = _el$3;
    return _el$3;
})();
const template75 = (()=>{
    var _el$4 = _tmpl$2();
    _$use(refConst, _el$4);
    return _el$4;
})();
const template76 = (()=>{
    var _el$5 = _tmpl$2();
    var _ref$3 = refUnknown;
    typeof _ref$3 === "function" ? _$use(_ref$3, _el$5) : refUnknown = _el$5;
    return _el$5;
})();
const template77 = (()=>{
    var _el$6 = _tmpl$3();
    _$setAttribute(_el$6, "true", true);
    return _el$6;
})();
const template78 = (()=>{
    var _el$7 = _tmpl$4();
    _$setAttribute(_el$7, "false", false);
    return _el$7;
})();
const template79 = (()=>{
    var _el$8 = _tmpl$2();
    _el$8.true = true;
    _el$8.false = false;
    return _el$8;
})();
const template80 = (()=>{
    var _el$9 = _tmpl$2();
    _$setAttribute(_el$9, "true", true);
    _$setAttribute(_el$9, "false", false);
    return _el$9;
})();
