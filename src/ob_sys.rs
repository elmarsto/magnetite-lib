use wasm_bindgen::prelude::*;
use js_sys::{ JsString, Promise, Object, Array };
use web_sys::{ HtmlElement };

// low-level APIs (plumbing, not porcelain; intentionally brutalist)

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Events)]
    pub type events;
    #[wasm_bindgen(method, js_class = "Events")]
    pub fn on(events: &events, eventName: &JsString);

    #[wasm_bindgen(js_name = App, extends = events)]
    pub type app;

    #[wasm_bindgen(js_name = Component, extends = events)]
    pub type component;
    #[wasm_bindgen(method, js_class = "Component", js_name = load)]
    pub fn component_load(this: &component);
    #[wasm_bindgen(method, js_class = "Component", js_name = unload)]
    pub fn component_unload(this: &component);
    #[wasm_bindgen(method, js_class = "Component", js_name = addChild)]
    pub fn component_add_child(this: &component, child: &component);
    #[wasm_bindgen(method, js_class = "Component", js_name = removeChild)]
    pub fn component_remove_child(this: &component, child: &component) -> bool;

    #[wasm_bindgen(js_name = Plugin, extends = component, extends = events)]
    pub type plugin;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addRibbonIcon)]
    pub fn plugin_add_ribbon_icon(this: &plugin, icon: &JsString, title: &JsString) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addStatusBarItem)]
    pub fn plugin_add_status_bar_item(this: &plugin) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addCommand)]
    pub fn plugin_add_command(this: &plugin, command: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = addSettingTab)]
    pub fn plugin_add_setting_tab(this: &plugin, setting_tab: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerView)]
    pub fn plugin_register_view(this: &plugin, view_creator: &Object);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn plugin_register_extensions(this: &plugin, extensions: &Array, viewType: &JsString);
    #[wasm_bindgen(method, js_class = "Plugin", js_name = registerExtensions)]
    pub fn plugin_load_data(this: &plugin) -> Promise;
    pub fn plugin_save_data(this: &plugin, data: JsValue) -> Promise;

    #[wasm_bindgen(extends = events)]
    pub type vault;
}
