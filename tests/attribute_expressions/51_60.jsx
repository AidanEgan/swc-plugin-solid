const template51 = <div bool:quack={"0"}>"0"</div>;
const template52 = <div bool:quack={undefined}>undefined</div>;
const template53 = <div bool:quack={null}>null</div>;
const template54 = <div bool:quack={boolTest()}>boolTest()</div>;
const template55 = <div bool:quack={boolTest}>boolTest</div>;
const template56 = <div bool:quack={boolTestBinding}>boolTestBinding</div>;
const template57 = <div bool:quack={boolTestObjBinding.value}>boolTestObjBinding.value</div>;
const template58 = <div bool:quack={()=>false}>fn</div>;

const template59 = <div before bool:quack="true">should have space before</div>;
const template60 = <div before bool:quack="true" after>should have space before/after</div>;
