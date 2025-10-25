import { template as _$template } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
import { mergeProps as _$mergeProps } from "solid-js/web";
import { use as _$use } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div>Hello"), _tmpl$2 = /*#__PURE__*/ _$template("<div>"), _tmpl$3 = /*#__PURE__*/ _$template("<div>From Parent"), _tmpl$4 = /*#__PURE__*/ _$template("<div> | <!> | <!> | <!> | <!> | "), _tmpl$5 = /*#__PURE__*/ _$template("<div> | <!> | <!> | "), _tmpl$6 = /*#__PURE__*/ _$template("<div> | <!> |  |  | <!> | "), _tmpl$7 = /*#__PURE__*/ _$template("<span>1"), _tmpl$8 = /*#__PURE__*/ _$template("<span>2"), _tmpl$9 = /*#__PURE__*/ _$template("<span>3");
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
                var _el$6 = _tmpl$2();
                _$insert(_el$6, content);
                return _el$6;
            }
        })), null);
        _$insert(_el$3, _$createComponent(Context.Consumer, {
            ref (r$) {
                var _ref$1 = props.consumerRef();
                typeof _ref$1 === "function" && _ref$1(r$);
            },
            children: (context)=>context
        }), null);
        return _el$3;
    })();
};
const template2 = _$createComponent(Child, {
    name: "Jake",
    get dynamic () {
        return state.data;
    },
    stale: state.data,
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
    ref: set,
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
    get fallback () {
        return _$createComponent(Loading, {});
    },
    children: (item)=>_$createComponent(Show, {
            get when () {
                return state.condition;
            },
            children: item
        })
});
const template7 = _$createComponent(Child, {
    get children () {
        return [
            _tmpl$2(),
            _$memo(()=>state.dynamic)
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
const template9 = _$createComponent(_garbage, {
    children: "Hi"
});
const template10 = (()=>{
    var _el$12 = _tmpl$4(), _el$13 = _el$12.firstChild, _el$14 = _el$13.nextSibling, _el$14 = _el$13.nextSibling, _el$16 = _el$14.nextSibling, _el$16 = _el$14.nextSibling, _el$18 = _el$16.nextSibling, _el$18 = _el$16.nextSibling, _el$20 = _el$18.nextSibling, _el$20 = _el$18.nextSibling;
    _$insert(_el$12, _$createComponent(Link, {
        children: "new"
    }), _el$13);
    _$insert(_el$12, _$createComponent(Link, {
        children: "comments"
    }), _el$14);
    _$insert(_el$12, _$createComponent(Link, {
        children: "show"
    }), _el$16);
    _$insert(_el$12, _$createComponent(Link, {
        children: "ask"
    }), _el$18);
    _$insert(_el$12, _$createComponent(Link, {
        children: "jobs"
    }), _el$20);
    _$insert(_el$12, _$createComponent(Link, {
        children: "submit"
    }), null);
    return _el$12;
})();
const template11 = (()=>{
    var _el$22 = _tmpl$5(), _el$23 = _el$22.firstChild, _el$24 = _el$23.nextSibling, _el$24 = _el$23.nextSibling, _el$26 = _el$24.nextSibling, _el$26 = _el$24.nextSibling;
    _$insert(_el$22, _$createComponent(Link, {
        children: "new"
    }), _el$23);
    _$insert(_el$22, _$createComponent(Link, {
        children: "comments"
    }), _el$24);
    _$insert(_el$22, _$createComponent(Link, {
        children: "show"
    }), _el$24);
    _$insert(_el$22, _$createComponent(Link, {
        children: "ask"
    }), _el$26);
    _$insert(_el$22, _$createComponent(Link, {
        children: "jobs"
    }), _el$26);
    _$insert(_el$22, _$createComponent(Link, {
        children: "submit"
    }), null);
    return _el$22;
})();
const template12 = (()=>{
    var _el$28 = _tmpl$6(), _el$29 = _el$28.firstChild, _el$30 = _el$29.nextSibling, _el$30 = _el$29.nextSibling, _el$32 = _el$30.nextSibling, _el$33 = _el$32.nextSibling, _el$34 = _el$33.nextSibling, _el$34 = _el$33.nextSibling;
    _$insert(_el$28, _$createComponent(Link, {
        children: "comments"
    }), _el$30);
    _$insert(_el$28, _$createComponent(Link, {
        children: "show"
    }), _el$34);
    return _el$28;
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
            _tmpl$7(),
            _tmpl$8(),
            _tmpl$9()
        ];
    }
});
const Template18 = _$createComponent(Pre, {
    get children () {
        return [
            _tmpl$7(),
            _tmpl$8(),
            _tmpl$9()
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
    get disabled () {
        return "t" in test;
    },
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
        get when () {
            const foo = test();
            if ("t" in foo) {
                return foo;
            }
        }
    }),
    _$createComponent(Component, {
        get when () {
            return ((val = 123)=>{
                return val * 2;
            })();
        }
    })
];
const template27 = _$createComponent(Component, {
    get when() {
      return prop.red ? "red" : "green";
    }
});
class Template28 {
    render() {
        return _$createComponent(Component, {
            get when () {
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
        _$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    }
    get get() {
        _$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    }
    set set(v) {
        _$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    }
    method() {
        _$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    }
    field = _$createComponent(this.component, {
        get method () {
            return this.method;
        },
        get comp () {
            return _$createComponent(this.another, {});
        }
    });
    fieldArrow = ()=>_$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    fieldFunction = function() {
        _$createComponent(this.component, {
            get method () {
                return this.method;
            }
        });
    };
}
const template30 = _$createComponent(Comp, {
    ref: binding
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
    ref: refConst
});
const template34 = _$createComponent(Comp, {
    ref (r$) {
        var _ref$1 = refUnknown;
        typeof _ref$1 === "function" ? _ref$1(r$) : refUnknown = r$;
    }
});
