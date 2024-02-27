use wasmtime::component::types::ComponentItem;
use wasmtime::component::{Component, Linker};
use wasmtime::{Engine, Config};

fn main() {
    let engine = Engine::new(&Config::new()).unwrap();
    let linker = Linker::<()>::new(&engine);
    let component = Component::from_file(&engine, "target/comp.wasm").unwrap();
    
    let component_ty = 
         linker
        .substituted_component_type(&component)
        .unwrap();
    let exports = component_ty.exports();
    for (name, item) in exports {
        dbg!((name, &item));
        match item {
            ComponentItem::ComponentInstance(instance) => {
                for item in instance.exports() {
                    dbg!(item);
                }
            }
            _ => ()
        } 
    }
}
