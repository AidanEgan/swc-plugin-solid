import { template as _$template } from "solid-js/web";
import { addEventListener as _$addEventListener } from "solid-js/web";
import { classList as _$classList } from "solid-js/web";
import { className as _$className } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { setBoolAttribute as _$setBoolAttribute } from "solid-js/web";
import { setStyleProperty as _$setStyleProperty } from "solid-js/web";
import { spread as _$spread } from "solid-js/web";
import { style as _$style } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div id=main><h1 class=base id=my-h1><a href=/>Welcome"), _tmpl$2 = /*#__PURE__*/ _$template("<div><div></div> <div></div><div>"), _tmpl$3 = /*#__PURE__*/ _$template("<div foo>"), _tmpl$4 = /*#__PURE__*/ _$template("<div>"), _tmpl$5 = /*#__PURE__*/ _$template('<div class="a b">'), _tmpl$6 = /*#__PURE__*/ _$template("<div style=margin-right:40px>"), _tmpl$7 = /*#__PURE__*/ _$template("<div onclick=console.log('hi')>"), _tmpl$8 = /*#__PURE__*/ _$template("<input type=checkbox>"), _tmpl$9 = /*#__PURE__*/ _$template("<div class=`a>`$`"), _tmpl$10 = /*#__PURE__*/ _$template('<button type=button class="static hi">Write'), _tmpl$11 = /*#__PURE__*/ _$template('<button class="a b c">Hi'), _tmpl$12 = /*#__PURE__*/ _$template('<div class="bg-red-500 flex flex-col">'), _tmpl$13 = /*#__PURE__*/ _$template("<div><input readOnly><input>"), _tmpl$14 = /*#__PURE__*/ _$template('<div data="hi" data2=">'), _tmpl$15 = /*#__PURE__*/ _$template("<a>"), _tmpl$16 = /*#__PURE__*/ _$template("<div><a>"), _tmpl$17 = /*#__PURE__*/ _$template("<div start=Hi>Hi"), _tmpl$18 = /*#__PURE__*/ _$template("<label><span>Input is</span><input><div>"), _tmpl$19 = /*#__PURE__*/ _$template('<div style="color: red;\n    background-color: blue !important;\n    border: 1px solid black;\n    font-size: 12px;" random="random1 random2\n    random3 random4" class="class1 class2\n    class3 class4\n    class5 class6">'), _tmpl$20 = /*#__PURE__*/ _$template("<button>"), _tmpl$21 = /*#__PURE__*/ _$template("<input value=10>"), _tmpl$22 = /*#__PURE__*/ _$template("<select><option>Red</option><option>Blue"), _tmpl$23 = /*#__PURE__*/ _$template("<div>empty string"), _tmpl$24 = /*#__PURE__*/ _$template("<div>js empty"), _tmpl$25 = /*#__PURE__*/ _$template("<div quack>hola"), _tmpl$26 = /*#__PURE__*/ _$template('<div quack>"hola js"'), _tmpl$27 = /*#__PURE__*/ _$template("<div quack>true"), _tmpl$28 = /*#__PURE__*/ _$template("<div>false"), _tmpl$29 = /*#__PURE__*/ _$template("<div quack>1"), _tmpl$30 = /*#__PURE__*/ _$template("<div>0"), _tmpl$31 = /*#__PURE__*/ _$template('<div quack>"1"'), _tmpl$32 = /*#__PURE__*/ _$template('<div>"0"'), _tmpl$33 = /*#__PURE__*/ _$template("<div>undefined"), _tmpl$34 = /*#__PURE__*/ _$template("<div>null"), _tmpl$35 = /*#__PURE__*/ _$template("<div>boolTest()"), _tmpl$36 = /*#__PURE__*/ _$template("<div>boolTest"), _tmpl$37 = /*#__PURE__*/ _$template("<div>boolTestBinding"), _tmpl$38 = /*#__PURE__*/ _$template("<div>boolTestObjBinding.value"), _tmpl$39 = /*#__PURE__*/ _$template("<div>fn"), _tmpl$40 = /*#__PURE__*/ _$template("<div before quack>should have space before"), _tmpl$41 = /*#__PURE__*/ _$template("<div before quack after>should have space before/after"), _tmpl$42 = /*#__PURE__*/ _$template("<div quack after>should have space before/after"), _tmpl$43 = /*#__PURE__*/ _$template("<img src>"), _tmpl$44 = /*#__PURE__*/ _$template("<div><img src>"), _tmpl$45 = /*#__PURE__*/ _$template("<img src loading=lazy>", true, false, false), _tmpl$46 = /*#__PURE__*/ _$template("<div><img src loading=lazy>", true, false, false), _tmpl$47 = /*#__PURE__*/ _$template("<iframe src>"), _tmpl$48 = /*#__PURE__*/ _$template("<div><iframe src>"), _tmpl$49 = /*#__PURE__*/ _$template("<iframe src loading=lazy>", true, false, false), _tmpl$50 = /*#__PURE__*/ _$template("<div><iframe src loading=lazy>", true, false, false), _tmpl$51 = /*#__PURE__*/ _$template("<div title=<u>data</u>>"), _tmpl$52 = /*#__PURE__*/ _$template("<div truestr=true truestrjs=true>"), _tmpl$53 = /*#__PURE__*/ _$template("<div falsestr=false falsestrjs=false>"), _tmpl$54 = /*#__PURE__*/ _$template("<math display=block><mrow>", false, false, true), _tmpl$55 = /*#__PURE__*/ _$template("<mrow><mi>x</mi><mo>=", false, false, true), _tmpl$56 = /*#__PURE__*/ _$template("<div style=background:red>"), _tmpl$57 = /*#__PURE__*/ _$template("<div style=background:red;color:green;margin:3;padding:0.4>"), _tmpl$58 = /*#__PURE__*/ _$template("<div style=background:red;color:green>");
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
    _$spread(_el$4, _$mergeProps(()=>getProps("test")), false, true);
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
const template11 = (()=>{
    var _el$17 = _tmpl$4();
    _$use(something, _el$17, ()=>true);
    _$use(another, _el$17, ()=>thing);
    _$use(zero, _el$17, ()=>0);
    return _el$17;
})();
const template12 = (()=>{
    var _el$18 = _tmpl$7();
    _el$18.htmlFor = thing;
    _el$18.number = 123;
    return _el$18;
})();
const template13 = (()=>{
    var _el$19 = _tmpl$8();
    _el$19.checked = true;
    return _el$19;
})();
const template14 = (()=>{
    var _el$20 = _tmpl$8();
    _$effect(()=>_el$20.checked = state.visible);
    return _el$20;
})();
const template15 = _tmpl$9();
const template16 = _tmpl$10();
const template17 = (()=>{
    var _el$25 = _tmpl$11();
    _$addEventListener(_el$25, "click", increment, true);
    return _el$25;
})();
const template18 = (()=>{
    var _el$27 = _tmpl$4();
    _$spread(_el$27, _$mergeProps({
        get [key()] () {
            return props.value;
        }
    }), false, false);
    return _el$27;
})();
const template19 = _tmpl$12();
const template20 = (()=>{
    var _el$29 = _tmpl$13(), _el$30 = _el$29.firstChild, _el$31 = _el$30.nextSibling;
    _$addEventListener(_el$30, "input", doSomething, true);
    _$addEventListener(_el$31, "input", doSomethingElse, true);
    _el$31.readOnly = value;
    _$effect((_p$)=>{
        var _v$7 = min(), _v$8 = max(), _v$9 = min(), _v$10 = max();
        _v$7 !== _p$.a && _$setAttribute(_el$30, "min", _p$.a = _v$7);
        _v$8 !== _p$.b && _$setAttribute(_el$30, "max", _p$.b = _v$8);
        _v$9 !== _p$.c && _$setAttribute(_el$31, "min", _p$.c = _v$9);
        _v$10 !== _p$.d && _$setAttribute(_el$31, "max", _p$.d = _v$10);
        return _p$;
    }, {
        a: undefined,
        b: undefined,
        c: undefined,
        d: undefined
    });
    _$effect(()=>_el$31.checked = s2());
    _$effect(()=>_el$30.value = s());
    return _el$29;
})();
const template21 = (()=>{
    var _el$32 = _tmpl$4();
    _$effect((_p$)=>_$style(_el$32, {
            a: "static",
            ...rest
        }, _p$));
    return _el$32;
})();
const template22 = _tmpl$14();
const template23 = (()=>{
    var _el$34 = _tmpl$4();
    _$insert(_el$34, "t" in test && "true");
    _$effect(()=>_el$34.disabled = "t" in test);
    return _el$34;
})();
const template24 = (()=>{
    var _el$35 = _tmpl$15();
    _$spread(_el$35, _$mergeProps(props, {
        something: ""
    }), false, false);
    return _el$35;
})();
const template25 = (()=>{
    var _el$36 = _tmpl$16(), _el$37 = _el$36.firstChild;
    _$spread(_el$37, _$mergeProps(props, {
        something: ""
    }), false, false);
    _$insert(_el$36, ()=>props.children, _el$37);
    return _el$36;
})();
const template26 = (()=>{
    var _el$38 = _tmpl$17();
    _$setAttribute(_el$38, "middle", middle);
    _$spread(_el$38, spread, false, true);
    return _el$38;
})();
const template27 = (()=>{
    var _el$40 = _tmpl$17();
    _$spread(_el$40, _$mergeProps(first, {
        middle: middle
    }, second), false, true);
    return _el$40;
})();
const template28 = (()=>{
    var _el$42 = _tmpl$18(), _el$43 = _el$42.firstChild, _el$44 = _el$43.firstChild, _el$45 = _el$43.nextSibling, _el$46 = _el$45.nextSibling;
    _$spread(_el$42, _$mergeProps(api), false, true);
    _$spread(_el$43, _$mergeProps(api), false, true);
    _$insert(_el$43, api() ? "checked" : "unchecked", null);
    _$spread(_el$45, _$mergeProps(api), false, false);
    _$spread(_el$46, _$mergeProps(api), false, false);
    return _el$42;
})();
const template29 = (()=>{
    var _el$47 = _tmpl$4();
    _$setAttribute(_el$47, "attribute", !!someValue);
    _$insert(_el$47, !!someValue);
    return _el$47;
})();
const template30 = _tmpl$19();
const template31 = (()=>{
    var _el$49 = _tmpl$4();
    _$effect(()=>_$setStyleProperty(_el$49, "background-color", getStore.itemProperties.color));
    return _el$49;
})();
const template32 = _tmpl$4();
const template33 = [
    (()=>{
        var _el$51 = _tmpl$20();
        _$effect(()=>_$className(_el$51, styles.button));
        return _el$51;
    })(),
    (()=>{
        var _el$52 = _tmpl$20();
        _$effect(()=>_$className(_el$52, styles["foo--bar"]));
        return _el$52;
    })(),
    (()=>{
        var _el$53 = _tmpl$20();
        _$effect(()=>_$className(_el$53, styles.foo.bar));
        return _el$53;
    })(),
    (()=>{
        var _el$54 = _tmpl$20();
        _$effect(()=>_$className(_el$54, styles[foo()]));
        return _el$54;
    })()
];
const template34 = (()=>{
    var _el$55 = _tmpl$4();
    _$use(something, _el$55, ()=>true);
    _$use(zero, _el$55, ()=>0);
    _$spread(_el$55, somethingElse, false, false);
    return _el$55;
})();
const template35 = (()=>{
    var _el$56 = _tmpl$4();
    var _ref$4 = a().b.c;
    typeof _ref$4 === "function" ? _$use(_ref$4, _el$56) : a().b.c = _el$56;
    return _el$56;
})();
const template36 = (()=>{
    var _el$57 = _tmpl$4();
    var _ref$5 = a().b?.c;
    typeof _ref$5 === "function" && _$use(_ref$5, _el$57);
    return _el$57;
})();
const template37 = (()=>{
    var _el$58 = _tmpl$4();
    var _ref$6 = a() ? b : c;
    typeof _ref$6 === "function" && _$use(_ref$6, _el$58);
    return _el$58;
})();
const template38 = (()=>{
    var _el$59 = _tmpl$4();
    var _ref$7 = a() ?? b;
    typeof _ref$7 === "function" && _$use(_ref$7, _el$59);
    return _el$59;
})();
const template39 = _tmpl$21();
const template40 = (()=>{
    var _el$61 = _tmpl$4();
    _$effect(()=>_$setStyleProperty(_el$61, "color", a()));
    return _el$61;
})();
const template41 = (()=>{
    var _el$62 = _tmpl$22(), _el$63 = _el$62.firstChild, _el$64 = _el$63.firstChild, _el$65 = _el$63.nextSibling;
    _$effect(()=>_el$65.value = Color.Blue);
    _$effect(()=>_el$63.value = Color.Red);
    _$effect(()=>_el$62.value = state.color);
    return _el$62;
})();
// bool:
function boolTest() {
    return true;
}
const boolTestBinding = false;
const boolTestObjBinding = {
    value: false
};
const template42 = _tmpl$23();
const template43 = _tmpl$24();
const template44 = _tmpl$25();
const template45 = _tmpl$26();
const template46 = _tmpl$27();
const template47 = _tmpl$28();
const template48 = _tmpl$29();
const template49 = _tmpl$30();
const template50 = _tmpl$31();
const template51 = _tmpl$32();
const template52 = _tmpl$33();
const template53 = _tmpl$34();
const template54 = (()=>{
    var _el$91 = _tmpl$35();
    _$effect(()=>_$setBoolAttribute(_el$91, "quack", boolTest()));
    return _el$91;
})();
const template55 = (()=>{
    var _el$93 = _tmpl$36();
    _$setBoolAttribute(_el$93, "quack", boolTest);
    return _el$93;
})();
const template56 = (()=>{
    var _el$95 = _tmpl$37();
    _$setBoolAttribute(_el$95, "quack", boolTestBinding);
    return _el$95;
})();
const template57 = (()=>{
    var _el$97 = _tmpl$38();
    _$effect(()=>_$setBoolAttribute(_el$97, "quack", boolTestObjBinding.value));
    return _el$97;
})();
const template58 = (()=>{
    var _el$99 = _tmpl$39();
    _$setBoolAttribute(_el$99, "quack", ()=>false);
    return _el$99;
})();
const template59 = _tmpl$40();
const template60 = _tmpl$41();
const template61 = _tmpl$42();
// this crash it for some reason- */ const template62 = <div bool:quack>really empty</div>;
const template63 = _tmpl$43();
const template64 = _tmpl$44();
const template65 = _tmpl$45();
const template66 = _tmpl$46();
const template67 = _tmpl$47();
const template68 = _tmpl$48();
const template69 = _tmpl$49();
const template70 = _tmpl$50();
const template71 = _tmpl$51();
const template72 = (()=>{
    var _el$120 = _tmpl$4();
    _$use(binding, _el$120);
    return _el$120;
})();
const template73 = (()=>{
    var _el$121 = _tmpl$4();
    var _ref$8 = binding.prop;
    typeof _ref$8 === "function" ? _$use(_ref$8, _el$121) : binding.prop = _el$121;
    return _el$121;
})();
const template74 = (()=>{
    var _el$122 = _tmpl$4();
    var _ref$9 = refFn;
    typeof _ref$9 === "function" ? _$use(_ref$9, _el$122) : refFn = _el$122;
    return _el$122;
})();
const template75 = (()=>{
    var _el$123 = _tmpl$4();
    _$use(refConst, _el$123);
    return _el$123;
})();
const template76 = (()=>{
    var _el$124 = _tmpl$4();
    var _ref$10 = refUnknown;
    typeof _ref$10 === "function" ? _$use(_ref$10, _el$124) : refUnknown = _el$124;
    return _el$124;
})();
const template77 = (()=>{
    var _el$125 = _tmpl$52();
    _$setAttribute(_el$125, "true", true);
    return _el$125;
})();
const template78 = (()=>{
    var _el$126 = _tmpl$53();
    _$setAttribute(_el$126, "false", false);
    return _el$126;
})();
const template79 = (()=>{
    var _el$127 = _tmpl$4();
    _el$127.true = true;
    _el$127.false = false;
    return _el$127;
})();
const template80 = (()=>{
    var _el$128 = _tmpl$4();
    _$setAttribute(_el$128, "true", true);
    _$setAttribute(_el$128, "false", false);
    return _el$128;
})();
const template81 = _tmpl$54();
const template82 = _tmpl$55();
const template83 = _tmpl$56();
const template84 = _tmpl$57();
const template85 = _tmpl$58();
const template86 = (()=>{
    var _el$139 = _tmpl$58();
    _$effect(()=>_$setStyleProperty(_el$139, "border", signal()));
    return _el$139;
})();
const template87 = (()=>{
    var _el$140 = _tmpl$58();
    _$setStyleProperty(_el$140, "border", somevalue);
    return _el$140;
})();
const template88 = (()=>{
    var _el$141 = _tmpl$58();
    _$effect(()=>_$setStyleProperty(_el$141, "border", some.access));
    return _el$141;
})();
const template89 = _tmpl$58();
_$delegateEvents([
    "click",
    "input"
]);
