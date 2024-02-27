
mod bindings {
    pub struct Foo {}
    pub struct MyResource {}
    ::wit_bindgen::generate!({
        exports: {
            "foo": Foo,
            "foo/myres": MyResource
        },
    });
}


impl bindings::exports::foo::Guest for bindings::Foo {
    fn make() -> bindings::exports::foo::OwnMyres {
        todo!()
    }
}
