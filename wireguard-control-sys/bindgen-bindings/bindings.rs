/* automatically generated by rust-bindgen 0.59.1 */

pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type sa_family_t = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::std::os::raw::c_char; 14usize],
}
#[test]
fn bindgen_test_layout_sockaddr() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr>(),
        2usize,
        concat!("Alignment of ", stringify!(sockaddr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr>())).sa_data as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr),
            "::",
            stringify!(sa_data)
        )
    );
}
pub type in_addr_t = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[test]
fn bindgen_test_layout_in_addr() {
    assert_eq!(
        ::std::mem::size_of::<in_addr>(),
        4usize,
        concat!("Size of: ", stringify!(in_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in_addr>())).s_addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in_addr),
            "::",
            stringify!(s_addr)
        )
    );
}
pub type in_port_t = u16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
}
#[test]
fn bindgen_test_layout_in6_addr__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr8 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr16 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<in6_addr__bindgen_ty_1>())).__u6_addr32 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr32)
        )
    );
}
impl Default for in6_addr__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for in6_addr__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "in6_addr__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_in6_addr() {
    assert_eq!(
        ::std::mem::size_of::<in6_addr>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<in6_addr>())).__in6_u as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr),
            "::",
            stringify!(__in6_u)
        )
    );
}
impl Default for in6_addr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for in6_addr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "in6_addr {{ __in6_u: {:?} }}", self.__in6_u)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout_sockaddr_in() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in>(),
        16usize,
        concat!("Size of: ", stringify!(sockaddr_in))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_addr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in>())).sin_zero as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in),
            "::",
            stringify!(sin_zero)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}
#[test]
fn bindgen_test_layout_sockaddr_in6() {
    assert_eq!(
        ::std::mem::size_of::<sockaddr_in6>(),
        28usize,
        concat!("Size of: ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        ::std::mem::align_of::<sockaddr_in6>(),
        4usize,
        concat!("Alignment of ", stringify!(sockaddr_in6))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_port as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_flowinfo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_flowinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_addr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sockaddr_in6>())).sin6_scope_id as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sockaddr_in6),
            "::",
            stringify!(sin6_scope_id)
        )
    );
}
impl Default for sockaddr_in6 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for sockaddr_in6 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "sockaddr_in6 {{ sin6_family: {:?}, sin6_port: {:?}, sin6_flowinfo: {:?}, sin6_addr: {:?}, sin6_scope_id: {:?} }}" , self . sin6_family , self . sin6_port , self . sin6_flowinfo , self . sin6_addr , self . sin6_scope_id)
    }
}
pub type wg_key = [u8; 32usize];
pub type wg_key_b64_string = [::std::os::raw::c_char; 45usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[test]
fn bindgen_test_layout_timespec64() {
    assert_eq!(
        ::std::mem::size_of::<timespec64>(),
        16usize,
        concat!("Size of: ", stringify!(timespec64))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec64>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec64))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec64>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec64),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timespec64>())).tv_nsec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec64),
            "::",
            stringify!(tv_nsec)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wg_allowedip {
    pub family: u16,
    pub __bindgen_anon_1: wg_allowedip__bindgen_ty_1,
    pub cidr: u8,
    pub next_allowedip: *mut wg_allowedip,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wg_allowedip__bindgen_ty_1 {
    pub ip4: in_addr,
    pub ip6: in6_addr,
}
#[test]
fn bindgen_test_layout_wg_allowedip__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wg_allowedip__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(wg_allowedip__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wg_allowedip__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wg_allowedip__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_allowedip__bindgen_ty_1>())).ip4 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_allowedip__bindgen_ty_1),
            "::",
            stringify!(ip4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_allowedip__bindgen_ty_1>())).ip6 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_allowedip__bindgen_ty_1),
            "::",
            stringify!(ip6)
        )
    );
}
impl Default for wg_allowedip__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for wg_allowedip__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "wg_allowedip__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_wg_allowedip() {
    assert_eq!(
        ::std::mem::size_of::<wg_allowedip>(),
        32usize,
        concat!("Size of: ", stringify!(wg_allowedip))
    );
    assert_eq!(
        ::std::mem::align_of::<wg_allowedip>(),
        8usize,
        concat!("Alignment of ", stringify!(wg_allowedip))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_allowedip>())).family as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_allowedip),
            "::",
            stringify!(family)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_allowedip>())).cidr as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_allowedip),
            "::",
            stringify!(cidr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_allowedip>())).next_allowedip as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_allowedip),
            "::",
            stringify!(next_allowedip)
        )
    );
}
impl Default for wg_allowedip {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for wg_allowedip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "wg_allowedip {{ family: {:?}, __bindgen_anon_1: {:?}, cidr: {:?}, next_allowedip: {:?} }}" , self . family , self . __bindgen_anon_1 , self . cidr , self . next_allowedip)
    }
}
impl wg_peer_flags {
    pub const WGPEER_REMOVE_ME: wg_peer_flags = wg_peer_flags(1);
}
impl wg_peer_flags {
    pub const WGPEER_REPLACE_ALLOWEDIPS: wg_peer_flags = wg_peer_flags(2);
}
impl wg_peer_flags {
    pub const WGPEER_HAS_PUBLIC_KEY: wg_peer_flags = wg_peer_flags(4);
}
impl wg_peer_flags {
    pub const WGPEER_HAS_PRESHARED_KEY: wg_peer_flags = wg_peer_flags(8);
}
impl wg_peer_flags {
    pub const WGPEER_HAS_PERSISTENT_KEEPALIVE_INTERVAL: wg_peer_flags = wg_peer_flags(16);
}
impl ::std::ops::BitOr<wg_peer_flags> for wg_peer_flags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        wg_peer_flags(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for wg_peer_flags {
    #[inline]
    fn bitor_assign(&mut self, rhs: wg_peer_flags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<wg_peer_flags> for wg_peer_flags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        wg_peer_flags(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for wg_peer_flags {
    #[inline]
    fn bitand_assign(&mut self, rhs: wg_peer_flags) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct wg_peer_flags(pub ::std::os::raw::c_uint);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wg_peer {
    pub flags: wg_peer_flags,
    pub public_key: wg_key,
    pub preshared_key: wg_key,
    pub endpoint: wg_peer__bindgen_ty_1,
    pub last_handshake_time: timespec64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub persistent_keepalive_interval: u16,
    pub first_allowedip: *mut wg_allowedip,
    pub last_allowedip: *mut wg_allowedip,
    pub next_peer: *mut wg_peer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wg_peer__bindgen_ty_1 {
    pub addr: sockaddr,
    pub addr4: sockaddr_in,
    pub addr6: sockaddr_in6,
}
#[test]
fn bindgen_test_layout_wg_peer__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wg_peer__bindgen_ty_1>(),
        28usize,
        concat!("Size of: ", stringify!(wg_peer__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wg_peer__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wg_peer__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer__bindgen_ty_1>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer__bindgen_ty_1),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer__bindgen_ty_1>())).addr4 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer__bindgen_ty_1),
            "::",
            stringify!(addr4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer__bindgen_ty_1>())).addr6 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer__bindgen_ty_1),
            "::",
            stringify!(addr6)
        )
    );
}
impl Default for wg_peer__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for wg_peer__bindgen_ty_1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "wg_peer__bindgen_ty_1 {{ union }}")
    }
}
#[test]
fn bindgen_test_layout_wg_peer() {
    assert_eq!(
        ::std::mem::size_of::<wg_peer>(),
        160usize,
        concat!("Size of: ", stringify!(wg_peer))
    );
    assert_eq!(
        ::std::mem::align_of::<wg_peer>(),
        8usize,
        concat!("Alignment of ", stringify!(wg_peer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).public_key as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(public_key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).preshared_key as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(preshared_key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).endpoint as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(endpoint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).last_handshake_time as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(last_handshake_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).rx_bytes as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(rx_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).tx_bytes as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(tx_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<wg_peer>())).persistent_keepalive_interval as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(persistent_keepalive_interval)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).first_allowedip as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(first_allowedip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).last_allowedip as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(last_allowedip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_peer>())).next_peer as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_peer),
            "::",
            stringify!(next_peer)
        )
    );
}
impl Default for wg_peer {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for wg_peer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write ! (f , "wg_peer {{ flags: {:?}, public_key: [{}], preshared_key: [{}], endpoint: {:?}, last_handshake_time: {:?}, rx_bytes: {:?}, tx_bytes: {:?}, persistent_keepalive_interval: {:?}, first_allowedip: {:?}, last_allowedip: {:?}, next_peer: {:?} }}" , self . flags , self . public_key . iter () . enumerate () . map (| (i , v) | format ! ("{}{:?}" , if i > 0 { ", " } else { "" } , v)) . collect :: < String > () , self . preshared_key . iter () . enumerate () . map (| (i , v) | format ! ("{}{:?}" , if i > 0 { ", " } else { "" } , v)) . collect :: < String > () , self . endpoint , self . last_handshake_time , self . rx_bytes , self . tx_bytes , self . persistent_keepalive_interval , self . first_allowedip , self . last_allowedip , self . next_peer)
    }
}
impl wg_device_flags {
    pub const WGDEVICE_REPLACE_PEERS: wg_device_flags = wg_device_flags(1);
}
impl wg_device_flags {
    pub const WGDEVICE_HAS_PRIVATE_KEY: wg_device_flags = wg_device_flags(2);
}
impl wg_device_flags {
    pub const WGDEVICE_HAS_PUBLIC_KEY: wg_device_flags = wg_device_flags(4);
}
impl wg_device_flags {
    pub const WGDEVICE_HAS_LISTEN_PORT: wg_device_flags = wg_device_flags(8);
}
impl wg_device_flags {
    pub const WGDEVICE_HAS_FWMARK: wg_device_flags = wg_device_flags(16);
}
impl ::std::ops::BitOr<wg_device_flags> for wg_device_flags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        wg_device_flags(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for wg_device_flags {
    #[inline]
    fn bitor_assign(&mut self, rhs: wg_device_flags) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<wg_device_flags> for wg_device_flags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        wg_device_flags(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for wg_device_flags {
    #[inline]
    fn bitand_assign(&mut self, rhs: wg_device_flags) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct wg_device_flags(pub ::std::os::raw::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wg_device {
    pub name: [::std::os::raw::c_char; 16usize],
    pub ifindex: u32,
    pub flags: wg_device_flags,
    pub public_key: wg_key,
    pub private_key: wg_key,
    pub fwmark: u32,
    pub listen_port: u16,
    pub first_peer: *mut wg_peer,
    pub last_peer: *mut wg_peer,
}
#[test]
fn bindgen_test_layout_wg_device() {
    assert_eq!(
        ::std::mem::size_of::<wg_device>(),
        112usize,
        concat!("Size of: ", stringify!(wg_device))
    );
    assert_eq!(
        ::std::mem::align_of::<wg_device>(),
        8usize,
        concat!("Alignment of ", stringify!(wg_device))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).ifindex as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(ifindex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).public_key as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(public_key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).private_key as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(private_key)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).fwmark as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(fwmark)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).listen_port as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(listen_port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).first_peer as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(first_peer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wg_device>())).last_peer as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(wg_device),
            "::",
            stringify!(last_peer)
        )
    );
}
impl Default for wg_device {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn wg_set_device(dev: *mut wg_device) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wg_get_device(
        dev: *mut *mut wg_device,
        device_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wg_add_device(device_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wg_del_device(device_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wg_free_device(dev: *mut wg_device);
}
extern "C" {
    pub fn wg_list_device_names() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn wg_key_to_base64(base64: *mut ::std::os::raw::c_char, key: *mut u8);
}
extern "C" {
    pub fn wg_key_from_base64(
        key: *mut u8,
        base64: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wg_key_is_zero(key: *mut u8) -> bool;
}
extern "C" {
    pub fn wg_generate_public_key(public_key: *mut u8, private_key: *mut u8);
}
extern "C" {
    pub fn wg_generate_private_key(private_key: *mut u8);
}
extern "C" {
    pub fn wg_generate_preshared_key(preshared_key: *mut u8);
}