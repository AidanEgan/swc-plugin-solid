import { template as _$template } from "solid-js/web";
import { addEventListener as _$addEventListener } from "solid-js/web";
import { createComponent as _$createComponent } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
var _tmpl$1 = /*#__PURE__*/ _$template("<button type=button>");
import { render } from "solid-js/web";
import { createSignal } from "solid-js";
function Counter() {
    const [count, setCount] = createSignal(1);
    return (()=>{
        var _el$0 = _tmpl$1();
        _$addEventListener(_el$0, "click", increment, true);
        _$insert(_el$0, ()=>count(1, 2));
        return _el$0;
    })();
}
render(()=>_$createComponent(Counter, {}), document.getElementById("app")!);
_$delegateEvents([
    "click"
]);
