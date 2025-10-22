const template41 = (
  <select value={state.color}>
    <option value={Color.Red}>Red</option>
    <option value={Color.Blue}>Blue</option>
  </select>
);

// bool:
function boolTest(){return true}
const boolTestBinding = false
const boolTestObjBinding = {value:false}

const template42 = <div bool:quack="">empty string</div>;
const template43 = <div bool:quack={""}>js empty</div>;
const template44 = <div bool:quack="hola">hola</div>;
const template45 = <div bool:quack={"hola js"}>"hola js"</div>;
const template46 = <div bool:quack={true}>true</div>;
const template47 = <div bool:quack={false}>false</div>;
const template48 = <div bool:quack={1}>1</div>;
const template49 = <div bool:quack={0}>0</div>;
const template50 = <div bool:quack={"1"}>"1"</div>;
