// browser.theme.update({
//   images: {},
//   properties: {},
//   colors: {
//     toolbar: "#282c34",
//     toolbar_text: "#abb2bf",
//     frame: "#202329",
//     tab_background_text: "#abb2bf",
//     toolbar_field: "#282c34",
//     toolbar_field_text: "#abb2bf",
//     tab_line: "#282c34",
//     popup: "#202329",
//     popup_text: "#abb2bf",
//     tab_loading: "#282c34",
//   },
// });

try {
  const port = browser.runtime.connectNative("dev.tombl.firefox_dyntheme");

  port.onMessage.addListener((msg) => {
    console.log("reloading theme");
    browser.theme.update(JSON.parse(msg));
  });
} catch (error) {
  console.error(error.toString());
}
