try {
  const port = browser.runtime.connectNative("dev.tombl.dyntheme");
  console.log(port.name);

  port.onMessage.addListener((msg) => {
    console.log("reloading theme");
    browser.theme.update(msg);
  });

  port.postMessage("reload");
} catch (error) {
  console.error(error.toString());
}
