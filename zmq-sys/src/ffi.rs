/* automatically generated by rust-bindgen */

#[repr(C)]
pub struct Struct_zmq_msg_t {
    pub unnamed_field1: [::libc::c_uchar, ..32u],
}
pub type zmq_msg_t = Struct_zmq_msg_t;
pub type zmq_free_fn = ::libc::c_void;
#[repr(C)]
pub struct Struct_Unnamed1 {
    pub event: uint16_t,
    pub value: int32_t,
}
pub type zmq_event_t = Struct_Unnamed1;
pub enum Struct_iovec { }
#[repr(C)]
pub struct Struct_Unnamed2 {
    pub socket: *mut ::libc::c_void,
    pub fd: ::libc::c_int,
    pub events: ::libc::c_short,
    pub revents: ::libc::c_short,
}
pub type zmq_pollitem_t = Struct_Unnamed2;
extern "C" { }
extern "C" {
    pub fn zmq_version(major: *mut ::libc::c_int, minor: *mut ::libc::c_int,
                       patch: *mut ::libc::c_int);
    pub fn zmq_errno() -> ::libc::c_int;
    pub fn zmq_strerror(errnum: ::libc::c_int) -> *const ::libc::c_char;
    pub fn zmq_ctx_new() -> *mut ::libc::c_void;
    pub fn zmq_ctx_term(context: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_ctx_shutdown(ctx_: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_ctx_set(context: *mut ::libc::c_void, option: ::libc::c_int,
                       optval: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_ctx_get(context: *mut ::libc::c_void, option: ::libc::c_int)
     -> ::libc::c_int;
    pub fn zmq_init(io_threads: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn zmq_term(context: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_ctx_destroy(context: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_msg_init(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_init_size(msg: *mut zmq_msg_t, size: size_t)
     -> ::libc::c_int;
    pub fn zmq_msg_init_data(msg: *mut zmq_msg_t, data: *mut ::libc::c_void,
                             size: size_t, ffn: *mut zmq_free_fn,
                             hint: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_msg_send(msg: *mut zmq_msg_t, s: *mut ::libc::c_void,
                        flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_msg_recv(msg: *mut zmq_msg_t, s: *mut ::libc::c_void,
                        flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_msg_close(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_move(dest: *mut zmq_msg_t, src: *mut zmq_msg_t)
     -> ::libc::c_int;
    pub fn zmq_msg_copy(dest: *mut zmq_msg_t, src: *mut zmq_msg_t)
     -> ::libc::c_int;
    pub fn zmq_msg_data(msg: *mut zmq_msg_t) -> *mut ::libc::c_void;
    pub fn zmq_msg_size(msg: *mut zmq_msg_t) -> size_t;
    pub fn zmq_msg_more(msg: *mut zmq_msg_t) -> ::libc::c_int;
    pub fn zmq_msg_get(msg: *mut zmq_msg_t, option: ::libc::c_int)
     -> ::libc::c_int;
    pub fn zmq_msg_set(msg: *mut zmq_msg_t, option: ::libc::c_int,
                       optval: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_socket(arg1: *mut ::libc::c_void, _type: ::libc::c_int)
     -> *mut ::libc::c_void;
    pub fn zmq_close(s: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_setsockopt(s: *mut ::libc::c_void, option: ::libc::c_int,
                          optval: *const ::libc::c_void, optvallen: size_t)
     -> ::libc::c_int;
    pub fn zmq_getsockopt(s: *mut ::libc::c_void, option: ::libc::c_int,
                          optval: *mut ::libc::c_void, optvallen: *mut size_t)
     -> ::libc::c_int;
    pub fn zmq_bind(s: *mut ::libc::c_void, addr: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn zmq_connect(s: *mut ::libc::c_void, addr: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn zmq_unbind(s: *mut ::libc::c_void, addr: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn zmq_disconnect(s: *mut ::libc::c_void, addr: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn zmq_send(s: *mut ::libc::c_void, buf: *const ::libc::c_void,
                    len: size_t, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_send_const(s: *mut ::libc::c_void, buf: *const ::libc::c_void,
                          len: size_t, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_recv(s: *mut ::libc::c_void, buf: *mut ::libc::c_void,
                    len: size_t, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_socket_monitor(s: *mut ::libc::c_void,
                              addr: *const ::libc::c_char,
                              events: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_sendmsg(s: *mut ::libc::c_void, msg: *mut zmq_msg_t,
                       flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_recvmsg(s: *mut ::libc::c_void, msg: *mut zmq_msg_t,
                       flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_sendiov(s: *mut ::libc::c_void, iov: *mut Struct_iovec,
                       count: size_t, flags: ::libc::c_int) -> ::libc::c_int;
    pub fn zmq_recviov(s: *mut ::libc::c_void, iov: *mut Struct_iovec,
                       count: *mut size_t, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn zmq_poll(items: *mut zmq_pollitem_t, nitems: ::libc::c_int,
                    timeout: ::libc::c_long) -> ::libc::c_int;
    pub fn zmq_proxy(frontend: *mut ::libc::c_void,
                     backend: *mut ::libc::c_void,
                     capture: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_proxy_steerable(frontend: *mut ::libc::c_void,
                               backend: *mut ::libc::c_void,
                               capture: *mut ::libc::c_void,
                               control: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn zmq_z85_encode(dest: *mut ::libc::c_char, data: *mut uint8_t,
                          size: size_t) -> *mut ::libc::c_char;
    pub fn zmq_z85_decode(dest: *mut uint8_t, string: *mut ::libc::c_char)
     -> *mut uint8_t;
    pub fn zmq_device(_type: ::libc::c_int, frontend: *mut ::libc::c_void,
                      backend: *mut ::libc::c_void) -> ::libc::c_int;
}
