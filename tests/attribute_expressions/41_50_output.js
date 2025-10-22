import { template as _$template } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<select><option>Red</option><option>Blue"), _tmpl$2 = /*#__PURE__*/ _$template("<div>empty string"), _tmpl$3 = /*#__PURE__*/ _$template("<div>js empty"), _tmpl$4 = /*#__PURE__*/ _$template("<div quack>hola"), _tmpl$5 = /*#__PURE__*/ _$template('<div quack>"hola js"'), _tmpl$6 = /*#__PURE__*/ _$template("<div quack>true"), _tmpl$7 = /*#__PURE__*/ _$template("<div>false"), _tmpl$8 = /*#__PURE__*/ _$template("<div quack>1"), _tmpl$9 = /*#__PURE__*/ _$template("<div>0"), _tmpl$10 = /*#__PURE__*/ _$template('<div quack>"1"');
const template41 = (()=>{
    var _el$0 = _tmpl$1(), _el$1 = _el$0.firstChild, _el$2 = _el$1.firstChild, _el$3 = _el$1.nextSibling;
    _$effect(()=>_el$3.value = Color.Blue);
    _$effect(()=>_el$1.value = Color.Red);
    _$effect(()=>_el$0.value = state.color);
    return _el$0;
})();
// bool:
function boolTest() {
    return true;
}
const boolTestBinding = false;
const boolTestObjBinding = {
    value: false
};
const template42 = _tmpl$2();
const template43 = _tmpl$3();
const template44 = _tmpl$4();
const template45 = _tmpl$5();
const template46 = _tmpl$6();
const template47 = _tmpl$7();
const template48 = _tmpl$8();
const template49 = _tmpl$9();
const template50 = _tmpl$10();
