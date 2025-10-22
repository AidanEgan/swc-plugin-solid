import { binding } from "somewhere";
const refConst = null;
const template71 = <div title="<u>data</u>"/>

const template72 = <div ref={binding} />;
const template73 = <div ref={binding.prop} />;
const template74 = <div ref={refFn} />
const template75 = <div ref={refConst} />

const template76 = <div ref={refUnknown} />

const template77 = <div true={true} truestr="true" truestrjs={"true"}/>
const template78 = <div false={false} falsestr="false" falsestrjs={"false"} />
const template79 = <div prop:true={true} prop:false={false}/>
const template80 = <div attr:true={true} attr:false={false}/>
