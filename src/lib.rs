#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[test]
fn test_connect_disconnect() {
    use std::ffi::{CStr, CString};

    unsafe {
        let url = CString::new("pulsar://localhost:6650").unwrap();
        let config = pulsar_client_configuration_create();
        let client = pulsar_client_create(url.as_ptr(), config);
        pulsar_client_configuration_free(config);

        let producer = Box::into_raw(Box::new(std::ptr::null_mut() as *mut pulsar_producer_t));
        let config = pulsar_producer_configuration_create();
        let topic = CString::new("persistent://public/default/my-topic").unwrap();
        let result = pulsar_client_create_producer(client, topic.as_ptr(), config, producer);

        assert_eq!(
            CStr::from_ptr(pulsar_result_str(result)).to_str().unwrap(),
            "Ok"
        );

        let result = pulsar_producer_close(*producer);
        pulsar_producer_free(*producer);
        drop(producer);

        assert_eq!(
            CStr::from_ptr(pulsar_result_str(result)).to_str().unwrap(),
            "Ok"
        );

        let result = pulsar_client_close(client);
        pulsar_client_free(client);
        drop(client);

        assert_eq!(
            CStr::from_ptr(pulsar_result_str(result)).to_str().unwrap(),
            "Ok"
        );
    }
}
