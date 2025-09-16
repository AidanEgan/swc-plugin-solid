import { template as _$template } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<div> Hello World! ");
const CustomComponent = ()=>{
    return _$createComponent(SomeCustom, _$mergeProps({
        data: "data",
        onClick: ()=>console.log("click")
    }, rest, {
        get children () {
            return _tmpl$1();
        }
    }));
};
