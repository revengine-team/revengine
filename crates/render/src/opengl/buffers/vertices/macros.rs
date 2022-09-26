#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    };
}

#[macro_export]
macro_rules! try_map_type_to_gl_type {
    ($ty: ty) => {
        match std::any::type_name::<$ty>() {
            "f32" => gl::FLOAT,
            "i32" => gl::INT,
            _ => panic!("{} is not supported", std::any::type_name::<$ty>())
            
        }
    };
}

#[macro_export]
macro_rules! implement_vertex_for {
    (struct $name: ident {
        $($field_name: ident : [$field_base_type: ty; $count_of_elements: expr],)*
    }) => {
        struct $name {
            $($field_name: [$field_base_type; $count_of_elements],)*
        }

        impl $name {
            pub fn new(
                $(
                    $field_name: [$field_base_type; $count_of_elements]
                ),*
            ) -> $name {
                $name {
                    $(
                        $field_name: $field_name
                    ),*
                }
            }
        }

        impl render::opengl::buffers::vertices::vertex::Vertex for $name {
            fn stride() -> usize {
                return std::mem::size_of::<$name>();
            }

            fn components() -> Vec<render::opengl::buffers::vertices::vertex::VertexComponent> {
                vec![
                   $(
                        render::opengl::buffers::vertices::vertex::VertexComponent::new(
                            $count_of_elements,
                            render::try_map_type_to_gl_type!($field_base_type),
                            render::offset_of!($name, $field_name)
                        )
                    ),*
                ]
            }
        }
    };
}
