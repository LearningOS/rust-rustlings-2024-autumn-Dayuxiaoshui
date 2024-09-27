/// # Safety
///
/// The `address` must be a valid pointer to a `u32` value.
/// The caller must ensure that the address is non-null, properly aligned, and
/// points to valid, mutable memory.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller must ensure that the pointer is valid, non-null,
    // and points to mutable memory that contains a `u32` value.
    let ptr = address as *mut u32;
    if !ptr.is_null() {
        *ptr = 0xAABBCCDD; // We modify the value to 0xAABBCCDD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
