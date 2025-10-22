import { template as _$template } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { setStyleProperty as _$setStyleProperty } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div>"), _tmpl$2 = /*#__PURE__*/ _$template("<button>"), _tmpl$3 = /*#__PURE__*/ _$template("<input value=10>");
const template31 = (()=>{
    var _el$0 = _tmpl$1();
    _$effect(()=>_$setStyleProperty(_el$0, "background-color", getStore.itemProperties.color));
    return _el$0;
})();
const template32 = _tmpl$1();
const template33 = [
    (()=>{
        var _el$2 = _tmpl$2();
        _$effect(()=>_$className(_el$2, styles.button));
        return _el$2;
    })(),
    (()=>{
        var _el$3 = _tmpl$2();
        _$effect(()=>_$className(_el$3, styles["foo--bar"]));
        return _el$3;
    })(),
    (()=>{
        var _el$4 = _tmpl$2();
        _$effect(()=>_$className(_el$4, styles.foo.bar));
        return _el$4;
    })(),
    (()=>{
        var _el$5 = _tmpl$2();
        _$effect(()=>_$className(_el$5, styles[foo()]));
        return _el$5;
    })()
];
const template34 = (()=>{
    var _el$6 = _tmpl$1();
    _$use(something, _el$6, ()=>true);
    _$use(zero, _el$6, ()=>0);
    _$spread(_el$6, somethingElse, false, false);
    return _el$6;
})();
const template35 = (()=>{
    var _el$7 = _tmpl$1();
    var _ref$1 = a().b.c;
    typeof _ref$1 === "function" ? _$use(_ref$1, _el$7) : a().b.c = _el$7;
    return _el$7;
})();
const template36 = (()=>{
    var _el$8 = _tmpl$1();
    var _ref$2 = a().b?.c;
    typeof _ref$2 === "function" && _$use(_ref$2, _el$8);
    return _el$8;
})();
const template37 = (()=>{
    var _el$9 = _tmpl$1();
    var _ref$3 = a() ? b : c;
    typeof _ref$3 === "function" && _$use(_ref$3, _el$9);
    return _el$9;
})();
const template38 = (()=>{
    var _el$10 = _tmpl$1();
    var _ref$4 = a() ?? b;
    typeof _ref$4 === "function" && _$use(_ref$4, _el$10);
    return _el$10;
})();
const template39 = _tmpl$3();
const template40 = (()=>{
    var _el$12 = _tmpl$1();
    _$effect(()=>_$setStyleProperty(_el$12, "color", a()));
    return _el$12;
})();
