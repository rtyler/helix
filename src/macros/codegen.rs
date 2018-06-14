#[macro_export]
macro_rules! codegen {
    { [ $($ast:tt)* ] } => {
        codegen! {
            type: top,
            classes: [],
            buffer: [ $($ast)* ]
        }

        codegen_init! { [ $($ast)* ] }
    };

    {
        type: top,
        classes: [ $($class:tt)* ],
        buffer: []
    } => {
        codegen! {
            type: done,
            classes: [ $($class)* ]
        }
    };

    {
        type: top,
        classes: [
            $($class:tt)*
        ],
        buffer: [
            {
                type: class,
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                meta: { pub: $pub:tt, reopen: $reopen:tt },
                struct: $struct:tt,
                methods: [ $($method:tt)* ]
            }
            $($rest:tt)*
        ]
    } => {
        codegen! {
            type: top,
            classes: [
                $($class)*
                {
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    struct: { codegen_struct! { pub: $pub, rust_name: $rust_name, ruby_name: $ruby_name, struct: $struct } },
                    methods: [ $( codegen_method! { $method } )* ]
                }
            ],
            buffer: [ $($rest)* ]
        }

        codegen_extra_impls!({
            type: class,
            rust_name: $rust_name,
            ruby_name: $ruby_name,
            meta: { pub: $pub, reopen: $reopen },
            struct: $struct,
            methods: [ $($method)* ]
        });
    };

    {
        type: done,
        classes: [ $(
            {
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                struct: { $($struct:tt)* },
                methods: [ $($method:tt)* ]
            }
        )* ]
    } => {
        $(
            $($struct)*

            impl $rust_name {
                $($method)*
            }
        )*
    };
}

#[macro_export]
macro_rules! codegen_pub_classes {
    {
            $({
                type: class,
                rust_name: $rust_name:tt,
                ruby_name: $ruby_name:tt,
                meta: { pub: $pub:tt, reopen: $reopen:tt },
                struct: $struct:tt,
                methods: [ $($method:tt)* ]
            })*
    } => {
        $(
            codegen_pub_classes! {
                type: class,
                rust_name: $rust_name,
                ruby_name: $ruby_name,
                pub: $pub
            }
        )*
    };

    {
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        pub: false
    } => {};

    {
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        pub: true
    } => {
        pub use self::init_native::$rust_name;
    };
}

#[macro_export]
macro_rules! codegen_struct {
    { pub: false, rust_name: $rust_name:tt, ruby_name: $ruby_name:tt, struct: () } => {
        codegen_struct! { pub: {}, rust_name: $rust_name, ruby_name: $ruby_name, struct: {} }
    };

    { pub: true, rust_name: $rust_name:tt, ruby_name: $ruby_name:tt, struct: () } => {
        codegen_struct! { pub: { pub }, rust_name: $rust_name, ruby_name: $ruby_name, struct: {} }
    };

    { pub: false, rust_name: $rust_name:tt, ruby_name: $ruby_name:tt, struct: { $($rest:tt)* } } => {
        codegen_struct! { pub: {}, rust_name: $rust_name, ruby_name: $ruby_name, struct: { $($rest)* } }
    };

    { pub: true, rust_name: $rust_name:tt, ruby_name: $ruby_name:tt, struct: { $($rest:tt)* } } => {
        codegen_struct! { pub: { pub }, rust_name: $rust_name, ruby_name: $ruby_name, struct: { $($rest)* } }
    };

    {
        pub: { $($pub:tt)* },
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        struct: { $($struct:tt)* }
    } => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        $($pub)* struct $rust_name {
            helix: $crate::Metadata,
            $($struct)*
        }

        #[allow(non_upper_case_globals)]
        static mut $rust_name: usize = 0;
    }
}

#[macro_export]
macro_rules! codegen_method {
    {
        {
            type: initializer,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            self: {
                ownership: {},
                name: $self:tt
            },
            args: [ $($arg:tt : $argty:ty),* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($self : $crate::Metadata, $($arg : $argty),*) -> $($ret)* $body
    };

    {
        {
            type: class_method,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            self: (),
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($($args)*) -> $($ret)* $body
    };

    {
        {
            type: instance_method,
            rust_name: $rust_name:tt,
            ruby_name: $ruby_name:tt,
            ruby_visibility: $ruby_visibility:tt,
            self: {
                ownership: { $($ownership:tt)* },
                name: $self:tt
            },
            args: [ $($args:tt)* ],
            ret: { $($ret:tt)* },
            body: $body:block
        }
    } => {
        pub fn $rust_name($($ownership)* $self, $($args)*) -> $($ret)* $body
    };
}

#[macro_export]
macro_rules! codegen_ruby_init {
    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: (),
        methods: $methods:tt
    }) => (
        impl $crate::InitRuby for $rust_name {
            fn init_ruby() {
                codegen_class_binding!({
                    type: class,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    meta: { pub: $pub, reopen: $reopen },
                    struct: (),
                    methods: $methods
                }, {
                    type: class,
                    rust_name: $rust_name,
                    ruby_name: $ruby_name,
                    meta: { pub: $pub, reopen: $reopen },
                    struct: (),
                    methods: $methods
                });
            }
        }
    );
}


#[macro_export]
macro_rules! codegen_extra_impls {
    ($class:tt) => (
        codegen_allocator!($class);
        codegen_coercions!($class);
        codegen_ruby_init!($class);
    )
}
