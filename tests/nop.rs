use std::{mem::zeroed, ptr::null_mut};

use liburing2_sys::{
    io_uring_cqe_get_data64, io_uring_get_sqe, io_uring_prep_nop, io_uring_queue_init,
    io_uring_sqe_set_data64, io_uring_submit, io_uring_wait_cqe,
};

#[test]
fn nop() {
    unsafe {
        let mut ring = zeroed();
        assert_eq!(io_uring_queue_init(1, &mut ring, 0), 0);
        let sqe = io_uring_get_sqe(&mut ring);
        assert!(!sqe.is_null());
        io_uring_prep_nop(sqe);
        io_uring_sqe_set_data64(sqe, 114514);
        assert_eq!(io_uring_submit(&mut ring), 1);
        let mut cqe = null_mut();
        assert_eq!(io_uring_wait_cqe(&mut ring, &mut cqe), 0);
        let data = io_uring_cqe_get_data64(cqe);
        assert_eq!(data, 114514);
    }
}
