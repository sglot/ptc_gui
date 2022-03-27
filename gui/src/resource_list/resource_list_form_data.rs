pub mod resource_list_form_data {
    use crate::resource_list::resource_list_typed_cache::resource_list_typed_cache::ResourceListTypedCache;

 
    pub struct ResourcseListFormData {
        pub typed_cache: ResourceListTypedCache,
    }

    impl ResourcseListFormData {
        pub fn new() -> Self {
            Self {
                typed_cache: ResourceListTypedCache::new(),
            }
        }
    }
}