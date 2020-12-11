use wasm_bindgen::prelude::*;
use js_sys::*;
use web_sys::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Events)]
    pub type App;
    pub type Component;
    // TODO: declare methods from class Component in obsidian.d.ts
    // Need load, onload,unload,onunload,add_child,remove_child,register,register_event,register_dom_event,register_scope_event,register_interval
    pub type EventRef;
    pub type Events;
    #[wasm_bindgen(method)]
    fn on(this: &Events) -> EventRef;
    #[wasm_bindgen(extends = Component)]
    pub type Plugin;
    #[wasm_bindgen(method, js_name = "addStatusBarItem")]
    fn add_status_bar_item(this: &Plugin) -> HtmlElement;
    #[wasm_bindgen(method)] // keep camelCased names where we wrap below for native rust strings
    fn addRibbonIcon(this: &Plugin, icon: &JsString, title: &JsString) -> HtmlElement;
    #[wasm_bindgen(extends = Events)]
    pub type Vault;
}

impl Plugin {
    pub fn vault(&self) -> Vault {
        Vault::from(Reflect::get(self,&JsValue::from("vault"))
          .expect("it returns the vault object"))
    }
    pub fn app(&self) -> App {
        App::from(Reflect::get(self,&JsValue::from("app")).expect("it returns the app object"))
    }
    // convention: keep camelCase on JS function or method when we are wrapping it
    // to convert between native rust strings and JsString
    pub fn add_ribbon_icon(&self, icon: &str, title: &str) {
        self.addRibbonIcon(&JsString::from(icon), &JsString::from(title));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
