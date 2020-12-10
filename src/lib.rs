use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Events)]
    pub type App;
    pub type Command;
    pub type Component;
    pub type Events;
    pub type Modal;
    #[wasm_bindgen(extends = Component)]
    pub type Plugin;
    #[wasm_bindgen(extends = Events)]
    pub type Vault;
    #[wasm_bindgen(extends = Component)]
    pub type View;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
