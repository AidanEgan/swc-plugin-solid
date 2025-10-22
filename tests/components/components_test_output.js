import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div>Hello"), _tmpl$2 = /*#__PURE__*/ _$template("<div>"), _tmpl$3 = /*#__PURE__*/ _$template("<div>From Parent"), _tmpl$4 = /*#__PURE__*/ _$template("<_garbage>Hi"), _tmpl$5 = /*#__PURE__*/ _$template("<div> | <!> | <!> | <!> | <!> | "), _tmpl$6 = /*#__PURE__*/ _$template("<div> | <!> | <!> | "), _tmpl$7 = /*#__PURE__*/ _$template("<div> | <!> |  |  | <!> | "), _tmpl$8 = /*#__PURE__*/ _$template("<span>1"), _tmpl$9 = /*#__PURE__*/ _$template("<span>2"), _tmpl$10 = /*#__PURE__*/ _$template("<span>3"), _tmpl$11 = /*#__PURE__*/ _$template("<this.component>"), _tmpl$12 = /*#__PURE__*/ _$template("<this.another>");
import { Show, binding } from "somewhere";
function refFn() {}
const refConst = null;
const Child = (props)=>{
    const [s, set] = createSignal();
    return [
        (()=>{
            var _el$0 = _tmpl$1(), _el$1 = _el$0.firstChild;
            var _ref$1 = props.ref;
            typeof _ref$1 === "function" ? _$use(_ref$1, _el$0) : props.ref = _el$0;
            _$insert(_el$0, ()=>props.name, null);
            return _el$0;
        })(),
        (()=>{
            var _el$2 = _tmpl$2();
            _$use(set, _el$2);
            _$insert(_el$2, ()=>props.children);
            return _el$2;
        })()
    ];
};
const template = (props)=>{
    let childRef;
    const { content } = props;
    return (()=>{
        var _el$3 = _tmpl$2();
        _$insert(_el$3, _$createComponent(Child, _$mergeProps({
            name: "John"
        }, props, {
            ref (r$) {
                var _ref$1 = childRef;
                typeof _ref$1 === "function" ? _ref$1(r$) : childRef = r$;
            },
            booleanProperty: true,
            get children () {
                return _tmpl$3();
            }
        })), null);
        _$insert(_el$3, _$createComponent(Child, _$mergeProps({
            name: "Jason"
        }, dynamicSpread, {
            ref (r$) {
                var _ref$1 = props.ref;
                typeof _ref$1 === "function" ? _ref$1(r$) : props.ref = r$;
            },
            get children () {
                return (()=>{
                    var _el$6 = _tmpl$2();
                    _$insert(_el$6, content);
                    return _el$6;
                })();
            }
        })), null);
        _$insert(_el$3, _$createComponent(Context.Consumer, {
            ref (r$) {
                var _ref$1 = props.consumerRef();
                typeof _ref$1 === "function" && _ref$1(r$);
            },
            get children () {
                return (context)=>context;
            }
        }), null);
        return _el$3;
    })();
};
const template2 = _$createComponent(Child, {
    name: "Jake",
    get dynamic () {
        return state.data;
    },
    get stale () {
        return state.data;
    },
    handleClick: clickHandler,
    get ["hyphen-ated"] () {
        return state.data;
    },
    ref: (el)=>e = el
});
const template3 = _$createComponent(Child, {
    get children () {
        return [
            _tmpl$2(),
            _tmpl$2(),
            _tmpl$2(),
            "After"
        ];
    }
});
const [s, set] = createSignal();
const template4 = _$createComponent(Child, {
    ref (r$) {
        var _ref$1 = set;
        typeof _ref$1 === "function" ? _ref$1(r$) : set = r$;
    },
    get children () {
        return _tmpl$2();
    }
});
const template5 = _$createComponent(Child, {
    get dynamic () {
        return state.dynamic;
    },
    get children () {
        return state.dynamic;
    }
});
// builtIns
const template6 = _$createComponent(For, {
    get each () {
        return state.list;
    },
    fallback: ()=>_$createComponent(Loading, {}),
    get children () {
        return (item)=>_$createComponent(Show, {
                get when () {
                    return state.condition;
                },
                get children () {
                    return item;
                }
            });
    }
});
const template7 = _$createComponent(Child, {
    get children () {
        return [
            _tmpl$2(),
            state.dynamic
        ];
    }
});
const template8 = _$createComponent(Child, {
    get children () {
        return [
            (item)=>item,
            (item)=>item
        ];
    }
});
const template9 = _tmpl$4();
const template10 = (()=>{
    var _el$14 = _tmpl$5(), _el$15 = _el$14.firstChild, _el$16 = _el$15.nextSibling, _el$16 = _el$15.nextSibling, _el$18 = _el$16.nextSibling, _el$18 = _el$16.nextSibling, _el$20 = _el$18.nextSibling, _el$20 = _el$18.nextSibling, _el$22 = _el$20.nextSibling, _el$22 = _el$20.nextSibling;
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "new";
        }
    }), _el$15);
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "comments";
        }
    }), _el$16);
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "show";
        }
    }), _el$18);
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "ask";
        }
    }), _el$20);
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "jobs";
        }
    }), _el$22);
    _$insert(_el$14, _$createComponent(Link, {
        get children () {
            return "submit";
        }
    }), null);
    return _el$14;
})();
const template11 = (()=>{
    var _el$24 = _tmpl$6(), _el$25 = _el$24.firstChild, _el$26 = _el$25.nextSibling, _el$26 = _el$25.nextSibling, _el$28 = _el$26.nextSibling, _el$28 = _el$26.nextSibling;
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "new";
        }
    }), _el$25);
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "comments";
        }
    }), _el$26);
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "show";
        }
    }), _el$26);
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "ask";
        }
    }), _el$28);
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "jobs";
        }
    }), _el$28);
    _$insert(_el$24, _$createComponent(Link, {
        get children () {
            return "submit";
        }
    }), null);
    return _el$24;
})();
const template12 = (()=>{
    var _el$30 = _tmpl$7(), _el$31 = _el$30.firstChild, _el$32 = _el$31.nextSibling, _el$32 = _el$31.nextSibling, _el$34 = _el$32.nextSibling, _el$35 = _el$34.nextSibling, _el$36 = _el$35.nextSibling, _el$36 = _el$35.nextSibling;
    _$insert(_el$30, _$createComponent(Link, {
        get children () {
            return "comments";
        }
    }), _el$32);
    _$insert(_el$30, _$createComponent(Link, {
        get children () {
            return "show";
        }
    }), _el$36);
    return _el$30;
})();
class Template13 {
    render() {
        _$createComponent(Component, {
            get prop () {
                return this.something;
            },
            onClick: ()=>this.shouldStay,
            get children () {
                return _$createComponent(Nested, {
                    get prop () {
                        return this.data;
                    },
                    get children () {
                        return this.content;
                    }
                });
            }
        });
    }
}
const Template14 = _$createComponent(Component, {
    get children () {
        return data();
    }
});
const Template15 = _$createComponent(Component, _$mergeProps(props));
const Template16 = _$createComponent(Component, _$mergeProps({
    something: something
}, props));
const Template17 = _$createComponent(Pre, {
    get children () {
        return [
            _tmpl$8(),
            _tmpl$9(),
            _tmpl$10()
        ];
    }
});
const Template18 = _$createComponent(Pre, {
    get children () {
        return [
            _tmpl$8(),
            _tmpl$9(),
            _tmpl$10()
        ];
    }
});
const Template19 = _$createComponent(Component, _$mergeProps(s.dynamic));
const Template20 = _$createComponent(Component, {
    class: prop.red ? "red" : "green"
});
const template21 = _$createComponent(Component, _$mergeProps({
    get [key()] () {
        return props.value;
    }
}));
const template22 = _$createComponent(Component, {
    passObject: {
        ...a
    }
});
const template23 = _$createComponent(Component, {
    disabled: "t" in test,
    get children () {
        return "t" in test && "true";
    }
});
const template24 = _$createComponent(Component, {
    get children () {
        return state.dynamic;
    }
});
const template25 = _$createComponent(Component, {
    get children () {
        return _tmpl$2();
    }
});
const template26 = [
    _$createComponent(Component, {
        when: ()=>{
            const foo = test();
            if ("t" in foo) {
                return foo;
            }
        }
    }),
    _$createComponent(Component, {
        when: (val = 123)=>{
            return val * 2;
        }
    })
];
const template27 = _$createComponent(Component, {
    when: ()=>prop.red ? "red" : "green"
});
class Template28 {
    render() {
        return _$createComponent(Component, {
            when: ()=>{
                const foo = this.value;
                if ("key" in foo) {
                    return foo;
                }
            }
        });
    }
}
class Template29 extends ParentComponent {
    constructor(){
        super();
        (()=>{
            var _el$51 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$51, "method", this.method));
            return _el$51;
        })();
    }
    get get() {
        (()=>{
            var _el$52 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$52, "method", this.method));
            return _el$52;
        })();
    }
    set set(v) {
        (()=>{
            var _el$53 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$53, "method", this.method));
            return _el$53;
        })();
    }
    method() {
        (()=>{
            var _el$54 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$54, "method", this.method));
            return _el$54;
        })();
    }
    field = (()=>{
        var _el$55 = _tmpl$11();
        _$setAttribute(_el$55, "comp", _tmpl$12());
        _$effect(()=>_$setAttribute(_el$55, "method", this.method));
        return _el$55;
    })();
    fieldArrow = ()=>(()=>{
            var _el$57 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$57, "method", this.method));
            return _el$57;
        })();
    fieldFunction = function() {
        (()=>{
            var _el$58 = _tmpl$11();
            _$effect(()=>_$setAttribute(_el$58, "method", this.method));
            return _el$58;
        })();
    };
}
const template30 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = binding;
        typeof _ref$1 === "function" ? _ref$1(r$) : binding = r$;
    }
});
const template31 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = binding.prop;
        typeof _ref$1 === "function" ? _ref$1(r$) : binding.prop = r$;
    }
});
const template32 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = refFn;
        typeof _ref$1 === "function" ? _ref$1(r$) : refFn = r$;
    }
});
const template33 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = refConst;
        typeof _ref$1 === "function" ? _ref$1(r$) : refConst = r$;
    }
});
const template34 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = refUnknown;
        typeof _ref$1 === "function" ? _ref$1(r$) : refUnknown = r$;
    }
});
