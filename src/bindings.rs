/* automatically generated by rust-bindgen 0.60.1 */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const TLS_TX: u32 = 1;
pub const TLS_RX: u32 = 2;
pub const TLS_TX_ZEROCOPY_RO: u32 = 3;
pub const TLS_RX_EXPECT_NO_PAD: u32 = 4;
pub const TLS_1_2_VERSION_MAJOR: u32 = 3;
pub const TLS_1_2_VERSION_MINOR: u32 = 3;
pub const TLS_1_3_VERSION_MAJOR: u32 = 3;
pub const TLS_1_3_VERSION_MINOR: u32 = 4;
pub const TLS_CIPHER_AES_GCM_128: u32 = 51;
pub const TLS_CIPHER_AES_GCM_128_IV_SIZE: u32 = 8;
pub const TLS_CIPHER_AES_GCM_128_KEY_SIZE: u32 = 16;
pub const TLS_CIPHER_AES_GCM_128_SALT_SIZE: u32 = 4;
pub const TLS_CIPHER_AES_GCM_128_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: u32 = 8;
pub const TLS_CIPHER_AES_GCM_256: u32 = 52;
pub const TLS_CIPHER_AES_GCM_256_IV_SIZE: u32 = 8;
pub const TLS_CIPHER_AES_GCM_256_KEY_SIZE: u32 = 32;
pub const TLS_CIPHER_AES_GCM_256_SALT_SIZE: u32 = 4;
pub const TLS_CIPHER_AES_GCM_256_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE: u32 = 8;
pub const TLS_CIPHER_AES_CCM_128: u32 = 53;
pub const TLS_CIPHER_AES_CCM_128_IV_SIZE: u32 = 8;
pub const TLS_CIPHER_AES_CCM_128_KEY_SIZE: u32 = 16;
pub const TLS_CIPHER_AES_CCM_128_SALT_SIZE: u32 = 4;
pub const TLS_CIPHER_AES_CCM_128_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE: u32 = 8;
pub const TLS_CIPHER_CHACHA20_POLY1305: u32 = 54;
pub const TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE: u32 = 12;
pub const TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE: u32 = 32;
pub const TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE: u32 = 0;
pub const TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE: u32 = 8;
pub const TLS_CIPHER_SM4_GCM: u32 = 55;
pub const TLS_CIPHER_SM4_GCM_IV_SIZE: u32 = 8;
pub const TLS_CIPHER_SM4_GCM_KEY_SIZE: u32 = 16;
pub const TLS_CIPHER_SM4_GCM_SALT_SIZE: u32 = 4;
pub const TLS_CIPHER_SM4_GCM_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE: u32 = 8;
pub const TLS_CIPHER_SM4_CCM: u32 = 56;
pub const TLS_CIPHER_SM4_CCM_IV_SIZE: u32 = 8;
pub const TLS_CIPHER_SM4_CCM_KEY_SIZE: u32 = 16;
pub const TLS_CIPHER_SM4_CCM_SALT_SIZE: u32 = 4;
pub const TLS_CIPHER_SM4_CCM_TAG_SIZE: u32 = 16;
pub const TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE: u32 = 8;
pub const TLS_SET_RECORD_TYPE: u32 = 1;
pub const TLS_GET_RECORD_TYPE: u32 = 2;
pub const TLS_CONF_BASE: u32 = 1;
pub const TLS_CONF_SW: u32 = 2;
pub const TLS_CONF_HW: u32 = 3;
pub const TLS_CONF_HW_RECORD: u32 = 4;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    fn test_field_fds_bits() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__kernel_fd_set>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fds_bits) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__kernel_fd_set),
                "::",
                stringify!(fds_bits)
            )
        );
    }
    test_field_fds_bits();
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    fn test_field_val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__kernel_fsid_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__kernel_fsid_t),
                "::",
                stringify!(val)
            )
        );
    }
    test_field_val();
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls_crypto_info {
    pub version: __u16,
    pub cipher_type: __u16,
}
#[test]
fn bindgen_test_layout_tls_crypto_info() {
    assert_eq!(
        ::std::mem::size_of::<tls_crypto_info>(),
        4usize,
        concat!("Size of: ", stringify!(tls_crypto_info))
    );
    assert_eq!(
        ::std::mem::align_of::<tls_crypto_info>(),
        2usize,
        concat!("Alignment of ", stringify!(tls_crypto_info))
    );
    fn test_field_version() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls_crypto_info>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls_crypto_info),
                "::",
                stringify!(version)
            )
        );
    }
    test_field_version();
    fn test_field_cipher_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls_crypto_info>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cipher_type) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(tls_crypto_info),
                "::",
                stringify!(cipher_type)
            )
        );
    }
    test_field_cipher_type();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub key: [::std::os::raw::c_uchar; 16usize],
    pub salt: [::std::os::raw::c_uchar; 4usize],
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_aes_gcm_128() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_aes_gcm_128>(),
        40usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_aes_gcm_128))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_aes_gcm_128>(),
        2usize,
        concat!("Alignment of ", stringify!(tls12_crypto_info_aes_gcm_128))
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_128),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_128),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_128),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_128),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_128),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls12_crypto_info_aes_gcm_256 {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub key: [::std::os::raw::c_uchar; 32usize],
    pub salt: [::std::os::raw::c_uchar; 4usize],
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_aes_gcm_256() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_aes_gcm_256>(),
        56usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_aes_gcm_256))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_aes_gcm_256>(),
        2usize,
        concat!("Alignment of ", stringify!(tls12_crypto_info_aes_gcm_256))
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_256>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_256),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_256>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_256),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_256>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_256),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_256>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_256),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_gcm_256>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_gcm_256),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls12_crypto_info_aes_ccm_128 {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub key: [::std::os::raw::c_uchar; 16usize],
    pub salt: [::std::os::raw::c_uchar; 4usize],
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_aes_ccm_128() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_aes_ccm_128>(),
        40usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_aes_ccm_128))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_aes_ccm_128>(),
        2usize,
        concat!("Alignment of ", stringify!(tls12_crypto_info_aes_ccm_128))
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_ccm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_ccm_128),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_ccm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_ccm_128),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_ccm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_ccm_128),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_ccm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_ccm_128),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_aes_ccm_128>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_aes_ccm_128),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
#[repr(C)]
#[derive(Debug)]
pub struct tls12_crypto_info_chacha20_poly1305 {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 12usize],
    pub key: [::std::os::raw::c_uchar; 32usize],
    pub salt: __IncompleteArrayField<::std::os::raw::c_uchar>,
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_chacha20_poly1305() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_chacha20_poly1305>(),
        56usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_chacha20_poly1305))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_chacha20_poly1305>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(tls12_crypto_info_chacha20_poly1305)
        )
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<tls12_crypto_info_chacha20_poly1305>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_chacha20_poly1305),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<tls12_crypto_info_chacha20_poly1305>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_chacha20_poly1305),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<tls12_crypto_info_chacha20_poly1305>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_chacha20_poly1305),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<tls12_crypto_info_chacha20_poly1305>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_chacha20_poly1305),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<tls12_crypto_info_chacha20_poly1305>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_chacha20_poly1305),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls12_crypto_info_sm4_gcm {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub key: [::std::os::raw::c_uchar; 16usize],
    pub salt: [::std::os::raw::c_uchar; 4usize],
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_sm4_gcm() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_sm4_gcm>(),
        40usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_sm4_gcm))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_sm4_gcm>(),
        2usize,
        concat!("Alignment of ", stringify!(tls12_crypto_info_sm4_gcm))
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_gcm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_gcm),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_gcm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_gcm),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_gcm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_gcm),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_gcm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_gcm),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_gcm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_gcm),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tls12_crypto_info_sm4_ccm {
    pub info: tls_crypto_info,
    pub iv: [::std::os::raw::c_uchar; 8usize],
    pub key: [::std::os::raw::c_uchar; 16usize],
    pub salt: [::std::os::raw::c_uchar; 4usize],
    pub rec_seq: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_tls12_crypto_info_sm4_ccm() {
    assert_eq!(
        ::std::mem::size_of::<tls12_crypto_info_sm4_ccm>(),
        40usize,
        concat!("Size of: ", stringify!(tls12_crypto_info_sm4_ccm))
    );
    assert_eq!(
        ::std::mem::align_of::<tls12_crypto_info_sm4_ccm>(),
        2usize,
        concat!("Alignment of ", stringify!(tls12_crypto_info_sm4_ccm))
    );
    fn test_field_info() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_ccm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).info) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_ccm),
                "::",
                stringify!(info)
            )
        );
    }
    test_field_info();
    fn test_field_iv() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_ccm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).iv) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_ccm),
                "::",
                stringify!(iv)
            )
        );
    }
    test_field_iv();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_ccm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_ccm),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_salt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_ccm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).salt) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_ccm),
                "::",
                stringify!(salt)
            )
        );
    }
    test_field_salt();
    fn test_field_rec_seq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<tls12_crypto_info_sm4_ccm>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rec_seq) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(tls12_crypto_info_sm4_ccm),
                "::",
                stringify!(rec_seq)
            )
        );
    }
    test_field_rec_seq();
}
pub const TLS_INFO_UNSPEC: _bindgen_ty_1 = 0;
pub const TLS_INFO_VERSION: _bindgen_ty_1 = 1;
pub const TLS_INFO_CIPHER: _bindgen_ty_1 = 2;
pub const TLS_INFO_TXCONF: _bindgen_ty_1 = 3;
pub const TLS_INFO_RXCONF: _bindgen_ty_1 = 4;
pub const TLS_INFO_ZC_RO_TX: _bindgen_ty_1 = 5;
pub const TLS_INFO_RX_NO_PAD: _bindgen_ty_1 = 6;
pub const __TLS_INFO_MAX: _bindgen_ty_1 = 7;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
