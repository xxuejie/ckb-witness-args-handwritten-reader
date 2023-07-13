use ckb_types::{bytes::Bytes, packed::WitnessArgs, prelude::*};
use core::ffi::c_void;
use proptest::prelude::*;

extern "C" {
    pub fn set_test_data(
        data: *const c_void,
        length: usize,
        syscall: usize,
        index: usize,
        source: usize,
    ) -> c_void;

    pub fn create_cursor(buf_length: usize, index: usize, source: usize) -> *mut c_void;
    pub fn destroy_cursor(reader: *mut c_void);

    pub fn alloc_witness_args_reader() -> *mut c_void;
    pub fn free_witness_args_reader(reader: *mut c_void);

    pub fn alloc_bytes_reader() -> *mut c_void;
    pub fn free_bytes_reader(reader: *mut c_void);

    pub fn cwhr_cursor_memcpy(cursor: *mut c_void, buf: *mut c_void) -> i32;

    pub fn cwhr_witness_args_reader_create(reader: *mut c_void, cursor: *mut c_void) -> i32;
    pub fn cwhr_witness_args_reader_verify(reader: *mut c_void, compatible: i32) -> i32;
    pub fn cwhr_witness_args_reader_has_input_type(reader: *mut c_void) -> i32;
    pub fn cwhr_witness_args_reader_has_output_type(reader: *mut c_void) -> i32;
    pub fn cwhr_witness_args_reader_input_type(reader: *mut c_void, input_type: *mut c_void)
        -> i32;

    pub fn cwhr_bytes_reader_length(reader: *mut c_void) -> u32;
    pub fn cwhr_bytes_reader_memcpy(reader: *mut c_void, buf: *mut c_void) -> i32;
}

proptest! {
    #[test]
    fn test_read_data(
        buf_length in 32..131072usize,
        data in prop::collection::vec(0..=255u8, 0..409600),
    ) {
        unsafe {
            set_test_data(
                data.as_ptr() as *const _,
                data.len(),
                2074,
                34,
                111,
            );
        }
        let cursor = unsafe {
            create_cursor(buf_length, 34, 111)
        };
        let mut read_data = vec![0; data.len()];
        assert_eq!(unsafe {
            cwhr_cursor_memcpy(cursor, read_data.as_mut_ptr() as *mut _)
        }, 0);
        assert_eq!(read_data, data);
        unsafe {
            destroy_cursor(cursor);
        }
    }

    #[test]
    fn test_witness_args_verify(
        buf_length in 32..131072usize,
        lock in prop::collection::vec(0..=255u8, 0..204800),
        input_type in prop::collection::vec(0..=255u8, 0..204800),
        output_type in prop::collection::vec(0..=255u8, 0..204800),
    ) {
        let mut builder = WitnessArgs::new_builder();
        if !lock.is_empty() {
            builder = builder.lock(Some(Bytes::from(lock)).pack());
        }
        if !input_type.is_empty() {
            builder = builder.input_type(Some(Bytes::from(input_type)).pack());
        }
        if !output_type.is_empty() {
            builder = builder.output_type(Some(Bytes::from(output_type)).pack());
        }
        let witness = builder.build().as_bytes();

        unsafe {
            set_test_data(
                witness.as_ptr() as *const _,
                witness.len(),
                2074,
                34,
                111,
            );
        }
        let cursor = unsafe {
            create_cursor(buf_length, 34, 111)
        };
        let reader = unsafe {
            alloc_witness_args_reader()
        };
        assert_eq!(unsafe {
            cwhr_witness_args_reader_create(reader, cursor)
        }, 0);
        let result = unsafe {
            cwhr_witness_args_reader_verify(reader, 0)
        };
        unsafe {
            free_witness_args_reader(reader);
            destroy_cursor(cursor);
        };
        assert_eq!(result, 0);
    }

    #[test]
    fn test_witness_args_verify_invalid(
        buf_length in 32..131072usize,
        lock in prop::collection::vec(0..=255u8, 0..204800),
        input_type in prop::collection::vec(0..=255u8, 0..204800),
        output_type in prop::collection::vec(0..=255u8, 0..204800),
        flip_bit in 0..64usize
    ) {
        let mut builder = WitnessArgs::new_builder();
        if !lock.is_empty() {
            builder = builder.lock(Some(Bytes::from(lock)).pack());
        }
        if !input_type.is_empty() {
            builder = builder.input_type(Some(Bytes::from(input_type)).pack());
        }
        if !output_type.is_empty() {
            builder = builder.output_type(Some(Bytes::from(output_type)).pack());
        }
        let mut witness = builder.build().as_bytes().to_vec();
        // Validation should fail when any bit in the first 16 bytes is flipped
        witness[flip_bit / 8] ^= 1 << (flip_bit % 8);

        unsafe {
            set_test_data(
                witness.as_ptr() as *const _,
                witness.len(),
                2074,
                34,
                111,
            );
        }
        let cursor = unsafe {
            create_cursor(buf_length, 34, 111)
        };
        let reader = unsafe {
            alloc_witness_args_reader()
        };
        assert_eq!(unsafe {
            cwhr_witness_args_reader_create(reader, cursor)
        }, 0);
        let result = unsafe {
            cwhr_witness_args_reader_verify(reader, 0)
        };
        unsafe {
            free_witness_args_reader(reader);
            destroy_cursor(cursor);
        };
        assert_ne!(result, 0);
    }

    #[test]
    fn test_witness_args_fetch(
        buf_length in 32..131072usize,
        lock in prop::collection::vec(0..=255u8, 0..204800),
        input_type in prop::collection::vec(0..=255u8, 1..204800),
    ) {
        let mut builder = WitnessArgs::new_builder();
        if !lock.is_empty() {
            builder = builder.lock(Some(Bytes::from(lock)).pack());
        }
        builder = builder.input_type(Some(Bytes::from(input_type.clone())).pack());
        let witness = builder.build().as_bytes();

        unsafe {
            set_test_data(
                witness.as_ptr() as *const _,
                witness.len(),
                2074,
                34,
                111,
            );
        }
        let cursor = unsafe {
            create_cursor(buf_length, 34, 111)
        };
        let reader = unsafe {
            alloc_witness_args_reader()
        };
        assert_eq!(unsafe {
            cwhr_witness_args_reader_create(reader, cursor)
        }, 0);
        assert_eq!(unsafe {
            cwhr_witness_args_reader_verify(reader, 0)
        }, 0);

        assert_eq!(unsafe {
            cwhr_witness_args_reader_has_input_type(reader)
        }, 1);
        assert_eq!(unsafe {
            cwhr_witness_args_reader_has_output_type(reader)
        }, 0);

        let input_type_reader = unsafe {
            alloc_bytes_reader()
        };
        assert_eq!(unsafe {
            cwhr_witness_args_reader_input_type(reader, input_type_reader)
        }, 0);
        assert_eq!(unsafe {
            cwhr_bytes_reader_length(input_type_reader)
        } as usize, input_type.len());

        let mut data = vec![0; input_type.len()];
        assert_eq!(unsafe {
            cwhr_bytes_reader_memcpy(input_type_reader, data.as_mut_ptr() as *mut _)
        }, 0);
        assert_eq!(data, input_type);

        unsafe {
            free_bytes_reader(input_type_reader);
            free_witness_args_reader(reader);
            destroy_cursor(cursor);
        }
    }
}
