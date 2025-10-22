const template31 = <div style={{ "background-color": getStore.itemProperties.color }} />;

const template32 = <div style={{ "background-color": undefined }} />;

const template33 = (
  <>
    <button class={styles.button}></button>
    <button class={styles["foo--bar"]}></button>
    <button class={styles.foo.bar}></button>
    <button class={styles[foo()]}></button>
  </>
);

const template34 = <div use:something {...somethingElse} use:zero={0} />;

const template35 = <div ref={a().b.c} />;

const template36 = <div ref={a().b?.c} />;

const template37 = <div ref={a() ? b : c} />;

const template38 = <div ref={a() ?? b} />;

const template39 = <input value={10} />;

const template40 = <div style={{ color: a() }} />;
