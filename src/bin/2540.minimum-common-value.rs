impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ptr_1 = 0;
        let mut ptr_2 = 0;

        while ptr_1 < nums1.len() && ptr_2 < nums2.len() {
            if nums1[ptr_1] < nums2[ptr_2] {
                ptr_1 += 1;
            } else if nums1[ptr_1] > nums2[ptr_2] {
                ptr_2 += 1;
            } else {
                return nums1[ptr_1];
            }
        }

        -1
    }
}
