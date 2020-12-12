use wasm_bindgen::prelude::*;
use js_sys::{ Promise, Object, Array };
use web_sys::{ HtmlElement };

// low-level APIs (plumbing, not porcelain; intentionally brutalist)
// note we preserve camelCase here; contextual cue that you're dealing with import
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    pub type Events;
    #[wasm_bindgen(method, js_class = "Events")]
    pub fn Events_on(events: &Events, eventName: &str);

    #[wasm_bindgen(extends = Events)]
    pub type App;

    #[wasm_bindgen(js_name = Component, extends = Events)]
    pub type Component;
    #[wasm_bindgen(method, js_class = "Component", js_name = load)]
    pub fn Component_load(this: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = unload)]
    pub fn Component_unload(this: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = addChild)]
    pub fn Component_addChild(this: &Component, child: &Component);
    #[wasm_bindgen(method, js_class = "Component", js_name = removeChild)]
    pub fn Component_removeChild(this: &Component, child: &Component) -> bool;

    #[wasm_bindgen(extends = Component, extends = Events)]
    pub type Plugin;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addRibbonIcon)]
    pub fn Plugin_addRibbonIcon(this: &Plugin, icon: &str, title: &str) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addStatusBarItem)]
    pub fn Plugin_addStatusBarItem(this: &Plugin) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addCommand)]
    pub fn Plugin_addCommand(this: &Plugin, command: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addSettingTab)]
    pub fn Plugin_addSettingTab(this: &Plugin, setting_tab: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerView)]
    pub fn Plugin_registerView(this: &Plugin, view_creator: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn Plugin_registerExtensions(this: &Plugin, extensions: &Array, viewType: &str);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn Plugin_loadData(this: &Plugin) -> Promise;
    pub fn Plugin_saveData(this: &Plugin, data: JsValue) -> Promise;

    #[wasm_bindgen(extends = Events)]
    pub type Vault;
}
