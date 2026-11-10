/// Mirror of one node of the `ClientLocatorBar` waypoint list: links, the waypoint handle and the waypoint pointer.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ListNodeLayout {
    pub next: usize,
    pub prev: usize,
    pub key: [u8; 16],
    pub waypoint: usize,
}
