/// Define the ingot entrypoint function.
#[macro_export]
macro_rules! ingot_init {
    ($init:expr) => {
        use $crate::Ingot;

        #[no_mangle]
        pub extern fn __ingot_init() -> *mut Ingot {
            let mut instance = $init;
            unsafe {
                Box::into_raw(Box::new(instance))
            }
        }

        #[no_mangle]
        pub extern fn __ingot_free(ptr: *mut Ingot) {
            unsafe {
                Box::from_raw(ptr);
            }
        }
    }
}
