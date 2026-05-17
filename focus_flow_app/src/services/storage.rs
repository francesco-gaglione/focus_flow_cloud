pub fn set_item(key: &str, value: &str) {
    #[cfg(feature = "web")]
    {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.set_item(key, value).unwrap();
    }

    #[cfg(not(feature = "web"))]
    {
        use dioxus_std::storage::StorageBacking;

        dioxus_std::storage::LocalStorage::set(key.to_string(), &value.to_string());
    }
}

pub fn remove_item(key: &str) {
    #[cfg(feature = "web")]
    {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.remove_item(key).unwrap();
    }

    #[cfg(not(feature = "web"))]
    {
        set_item(key, "");
    }
}

pub fn get_item(key: &str) -> Option<String> {
    #[cfg(feature = "web")]
    {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.get_item(key).unwrap().filter(|v| !v.is_empty())
    }

    #[cfg(not(feature = "web"))]
    {
        use dioxus_std::storage::StorageBacking;

        dioxus_std::storage::LocalStorage::get(&key.to_string()).filter(|v: &String| !v.is_empty())
    }
}
