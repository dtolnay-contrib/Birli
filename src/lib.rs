// #![feature(static_nobundle)]
// mod aoflagger_bindings {
//     #![allow(non_upper_case_globals)]
//     #![allow(non_camel_case_types)]
//     #![allow(non_snake_case)]
//     #![allow(clippy::all)]
//     include!(concat!(env!("OUT_DIR"), "/aoflagger_bindings.rs"));
// }

// mod glue;
mod glue;

#[cfg(test)]
mod tests {
    use super::glue::aoflagger_AOFlagger_GetVersion;
    use std::os::raw::c_short;

    #[test]
    fn test_valid_aoflagger_version() {
        let mut major: c_short = -1;
        let mut minor: c_short = -1;
        let mut sub_minor: c_short = -1;
        // const _void: c_void = c_void::__variant1;
        // const vtable: aoflagger_StatusListener__bindgen_vtable =
        //     aoflagger_StatusListener__bindgen_vtable(_void);
        // const statusListener: aoflagger_StatusListener = aoflagger_StatusListener {vtable_: &vtable};
        // let aoflagger: aoflagger_AOFlagger = aoflagger_AOFlagger { _statusListener: &mut statusListener };
        unsafe {
            aoflagger_AOFlagger_GetVersion(
                &mut major,
                &mut minor,
                &mut sub_minor,
            );
        }
        assert!(major != -1);
        assert!(minor != -1);
        assert!(sub_minor != -1);
    }
}
