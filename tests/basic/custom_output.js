import { createComponent as _$createComponent } from "solid-js/web";
const CustomComponent = ()=>{
    return _$createComponent(SomeCustom, {
        data: "data",
        onClick: ()=>console.log("click")
    });
};
const CustomComponent2 = ()=>{
    return _$createComponent(SomeOtherCustom, {});
};
