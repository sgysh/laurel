use core;

#[repr(C)]
pub struct ChunkHeader {
    next: Option<usize>,
    chunk_size: usize,
    dummy_size: usize, // for debug
}

impl ChunkHeader {
    pub const fn empty() -> Self {
        Self {
            next: None,
            chunk_size: 0,
            dummy_size: 0,
        }
    }

    pub unsafe fn new(heap_start: usize, heap_size: usize) -> ChunkHeader {
        let new_chunk_head = ChunkHeader {
            next: None,
            chunk_size: heap_size,
            dummy_size: 0,
        };
        (heap_start as *mut ChunkHeader).write(new_chunk_head);
        ChunkHeader {
            next: Some(heap_start),
            chunk_size: 0,
            dummy_size: 0,
        }
    }

    pub unsafe fn add(&mut self, request_size: usize) -> Option<*mut u8> {
        let mut prev_head_ptr = self as *mut ChunkHeader;
        if self.next.is_none() {
            return None; // no free chunk;
        }
        let mut head_ptr = self.next.unwrap() as *mut ChunkHeader;
        loop {
            //
            // HH: header
            // U: used
            // _: free
            // D: dummy
            //
            // HHUU: used chunk
            // HH__: free chunk
            //
            if request_size + core::mem::size_of::<ChunkHeader>() * 2 <= (*head_ptr).chunk_size {
                //
                // HHUUHH____HHUU ==> HHUUHHUHH_HHUU
                //
                let new_chunk_head = ChunkHeader {
                    next: (*head_ptr).next,
                    chunk_size: (*head_ptr).chunk_size - request_size
                        - core::mem::size_of::<ChunkHeader>(),
                    dummy_size: 0,
                };
                let prev_chunk_head = ChunkHeader {
                    next: Some(
                        head_ptr as usize + request_size + core::mem::size_of::<ChunkHeader>(),
                    ),
                    chunk_size: (*prev_head_ptr).chunk_size,
                    dummy_size: 0,
                };
                let use_chunk_head = ChunkHeader {
                    next: None, // unused field
                    chunk_size: request_size + core::mem::size_of::<ChunkHeader>(),
                    dummy_size: 0,
                };
                prev_head_ptr.write(prev_chunk_head);
                ((head_ptr as usize + request_size + core::mem::size_of::<ChunkHeader>())
                    as *mut ChunkHeader)
                    .write(new_chunk_head);
                head_ptr.write(use_chunk_head);
                break;
            } else if request_size + core::mem::size_of::<ChunkHeader>() <= (*head_ptr).chunk_size {
                //
                // HHUUHH__HHUU ==> HHUUHHUDHHUU
                //
                let prev_chunk_head = ChunkHeader {
                    next: (*head_ptr).next,
                    chunk_size: (*prev_head_ptr).chunk_size,
                    dummy_size: 0,
                };
                let use_chunk_head = ChunkHeader {
                    next: None,                         // unused field
                    chunk_size: (*head_ptr).chunk_size, // include dummy space
                    dummy_size: (*head_ptr).chunk_size - request_size
                        - core::mem::size_of::<ChunkHeader>(),
                };
                prev_head_ptr.write(prev_chunk_head);
                head_ptr.write(use_chunk_head);
                break;
            }

            if (*head_ptr).next.is_none() {
                return None;
            }

            prev_head_ptr = head_ptr;
            head_ptr = (*head_ptr).next.unwrap() as *mut ChunkHeader;
        }

        Some(
            ((head_ptr as usize + core::mem::size_of::<ChunkHeader>()) as *mut ChunkHeader)
                as *mut u8,
        )
    }

    pub unsafe fn del(&mut self, ptr: *mut u8, request_size: usize) {
        let head_ptr = (ptr as usize - core::mem::size_of::<ChunkHeader>()) as *mut ChunkHeader;
        assert!(
            (*head_ptr).chunk_size - (*head_ptr).dummy_size - core::mem::size_of::<ChunkHeader>()
                == request_size
        );
        let real_size = (*head_ptr).chunk_size - core::mem::size_of::<ChunkHeader>();

        let mut prev_free_head_ptr = self as *mut ChunkHeader;
        while (*prev_free_head_ptr).next.is_some()
            && (*prev_free_head_ptr).next.unwrap() < head_ptr as usize
        {
            prev_free_head_ptr = (*prev_free_head_ptr).next.unwrap() as *mut ChunkHeader;
        }
        let next_free_head_ptr;
        if (*prev_free_head_ptr).next.is_some() {
            next_free_head_ptr = (*prev_free_head_ptr).next.unwrap() as *mut ChunkHeader;
        } else {
            // no next free space
            next_free_head_ptr = 0 as *mut ChunkHeader; // dummy. maybe unuse.
        }

        //
        // H: header
        // U: used
        // _: free
        //
        // HUU: used chunk
        // H__: free chunk
        //
        if prev_free_head_ptr as usize + (*prev_free_head_ptr).chunk_size == head_ptr as usize
            && ptr as usize + real_size == next_free_head_ptr as usize
        {
            //
            // H__HUUH__ ==> H________
            //
            let new_prev_free_head = ChunkHeader {
                next: (*next_free_head_ptr).next,
                chunk_size: (*prev_free_head_ptr).chunk_size + (*head_ptr).chunk_size
                    + (*next_free_head_ptr).chunk_size,
                dummy_size: 0,
            };
            prev_free_head_ptr.write(new_prev_free_head);
        } else if prev_free_head_ptr as usize + (*prev_free_head_ptr).chunk_size
            == head_ptr as usize
        {
            //
            // H__HUUHUU ==> H_____HUU
            //
            let new_prev_free_head = ChunkHeader {
                next: (*prev_free_head_ptr).next,
                chunk_size: (*prev_free_head_ptr).chunk_size + (*head_ptr).chunk_size,
                dummy_size: 0,
            };
            prev_free_head_ptr.write(new_prev_free_head);
        } else if ptr as usize + real_size == next_free_head_ptr as usize {
            //
            // HUUHUUH__ ==> HUUH_____
            // or
            // [HEAP START]HUUH__ ==> [HEAP START]H_____
            //
            let new_prev_free_head = ChunkHeader {
                next: Some(head_ptr as usize),
                chunk_size: (*prev_free_head_ptr).chunk_size,
                dummy_size: 0,
            };
            let new_head = ChunkHeader {
                next: (*next_free_head_ptr).next,
                chunk_size: (*head_ptr).chunk_size + (*next_free_head_ptr).chunk_size,
                dummy_size: 0,
            };
            prev_free_head_ptr.write(new_prev_free_head);
            head_ptr.write(new_head);
        } else {
            //
            // HUUHUUHUU ==> HUUH__HUU
            // or
            // [HEAP START]HUUHUU ==> [HEAP START]H__HUU
            //
            let new_prev_free_head = ChunkHeader {
                next: Some(head_ptr as usize),
                chunk_size: (*prev_free_head_ptr).chunk_size,
                dummy_size: 0,
            };
            let new_head = ChunkHeader {
                next: (*prev_free_head_ptr).next,
                chunk_size: (*head_ptr).chunk_size,
                dummy_size: 0,
            };
            prev_free_head_ptr.write(new_prev_free_head);
            head_ptr.write(new_head);
        }
    }
}
