use wasm_bindgen::prelude::*;
use js_sys::{ JsString };
use web_sys::{ HtmlElement };

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Events)]
    pub type App;
    // TODO: declare methods from class Component in obsidian.d.ts
    // Need load, onload,unload,onunload,add_child,remove_child,register,register_event,register_dom_event,register_scope_event,register_interval

    pub type EventRef;

    pub type Events;
    #[wasm_bindgen(method)]
    pub fn on(this: &Events, eventName: &JsString) -> EventRef;

    #[wasm_bindgen(extends = Component)]
    pub type Plugin;
    #[wasm_bindgen(method, js_name = "addStatusBarItem")]
    pub fn add_status_bar_item(this: &Plugin) -> HtmlElement;
    #[wasm_bindgen(method, js_class = "Plugin", js_name = "addRibbonIcon")]
    pub fn add_ribbon_icon(this: &Plugin, icon: &JsString, title: &JsString) -> web_sys::HtmlElement;

    #[wasm_bindgen(extends = Events)]
    pub type Vault;
}