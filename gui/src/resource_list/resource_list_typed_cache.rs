pub mod resource_list_typed_cache {
    pub struct ResourceListTypedCache {
        pub resource_list: Vec<String>,
    }

    impl ResourceListTypedCache {
        pub fn new() -> Self {
            Self {
                resource_list: vec![],
            }
        }

        pub fn resource_list(&self) -> Vec<String> {
            self.resource_list.clone()
        }

        pub fn set_resource_list(&mut self, resource_list: Vec<String>) {
            self.resource_list = resource_list;
        }

        pub fn drop_resource_list(&mut self) {
            self.resource_list = vec![];
        }
    }
}
