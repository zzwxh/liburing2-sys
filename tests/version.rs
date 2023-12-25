use liburing2_sys::{io_uring_major_version, io_uring_minor_version};

#[test]
fn version() {
    unsafe {
        assert_eq!(io_uring_major_version(), 2);
        assert_eq!(io_uring_minor_version(), 5);
    }
}
