use std::arch::x86_64::{
    __m256i, _mm256_add_epi64, _mm256_add_epi8, _mm256_and_si256, _mm256_extract_epi64,
    _mm256_or_si256, _mm256_sad_epu8, _mm256_set1_epi8, _mm256_setr_epi8, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_slli_epi64, _mm256_srli_epi32, _mm256_xor_si256,
};

#[target_feature(enable = "avx2")]
unsafe fn carry_save_adder(h: *mut __m256i, l: *mut __m256i, a: __m256i, b: __m256i, c: __m256i) {
    let u = _mm256_xor_si256(a, b);
    *h = _mm256_or_si256(_mm256_and_si256(a, b), _mm256_and_si256(u, c));
    *l = _mm256_xor_si256(u, c);
}

#[target_feature(enable = "avx2")]
unsafe fn count(v: __m256i) -> __m256i {
    let lookup = _mm256_setr_epi8(
        0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3,
        3, 4,
    );
    let low_mask = _mm256_set1_epi8(0x0f);
    let lo = _mm256_and_si256(v, low_mask);
    let hi = _mm256_and_si256(_mm256_srli_epi32(v, 4), low_mask);
    let popcnt1 = _mm256_shuffle_epi8(lookup, lo);
    let popcnt2 = _mm256_shuffle_epi8(lookup, hi);
    let total = _mm256_add_epi8(popcnt1, popcnt2);
    _mm256_sad_epu8(total, _mm256_setzero_si256())
}

#[target_feature(enable = "avx2")]
unsafe fn avx2_harvey_seal_popcnt(x: *const __m256i, y: *const __m256i, size: usize) -> u64 {
    let mut total = _mm256_setzero_si256();
    let mut ones = _mm256_setzero_si256();
    let mut twos = _mm256_setzero_si256();
    let mut fours = _mm256_setzero_si256();
    let mut eights = _mm256_setzero_si256();
    let mut sixteens = _mm256_setzero_si256();
    let mut twos_a = _mm256_setzero_si256();
    let mut fours_a = _mm256_setzero_si256();
    let mut eights_a = _mm256_setzero_si256();
    let mut twos_b = _mm256_setzero_si256();
    let mut fours_b = _mm256_setzero_si256();
    let mut eights_b = _mm256_setzero_si256();
    let mut i = 0;
    while i < size {
        carry_save_adder(
            &mut twos_a,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i), *y.add(i)),
            _mm256_xor_si256(*x.add(i + 1), *y.add(i + 1)),
        );
        carry_save_adder(
            &mut twos_b,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 2), *y.add(i + 2)),
            _mm256_xor_si256(*x.add(i + 3), *y.add(i + 3)),
        );
        carry_save_adder(&mut fours_a, &mut twos, twos, twos_a, twos_b);
        carry_save_adder(
            &mut twos_a,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 4), *y.add(i + 4)),
            _mm256_xor_si256(*x.add(i + 5), *y.add(i + 5)),
        );
        carry_save_adder(
            &mut twos_b,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 6), *y.add(i + 6)),
            _mm256_xor_si256(*x.add(i + 7), *y.add(i + 7)),
        );
        carry_save_adder(&mut fours_b, &mut twos, twos, twos_a, twos_b);
        carry_save_adder(&mut eights_a, &mut fours, fours, fours_a, fours_b);
        carry_save_adder(
            &mut twos_a,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 8), *y.add(i + 8)),
            _mm256_xor_si256(*x.add(i + 9), *y.add(i + 9)),
        );
        carry_save_adder(
            &mut twos_b,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 10), *y.add(i + 10)),
            _mm256_xor_si256(*x.add(i + 11), *y.add(i + 11)),
        );
        carry_save_adder(&mut fours_a, &mut twos, twos, twos_a, twos_b);
        carry_save_adder(
            &mut twos_a,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 12), *y.add(i + 12)),
            _mm256_xor_si256(*x.add(i + 13), *y.add(i + 13)),
        );
        carry_save_adder(
            &mut twos_b,
            &mut ones,
            ones,
            _mm256_xor_si256(*x.add(i + 14), *y.add(i + 14)),
            _mm256_xor_si256(*x.add(i + 15), *y.add(i + 15)),
        );
        carry_save_adder(&mut fours_b, &mut twos, twos, twos_a, twos_b);
        carry_save_adder(&mut eights_b, &mut fours, fours, fours_a, fours_b);
        carry_save_adder(&mut sixteens, &mut eights, eights, eights_a, eights_b);
        total = _mm256_add_epi64(total, count(sixteens));
        i += 16;
    }
    // final reduce
    total = _mm256_slli_epi64(total, 4);
    total = _mm256_add_epi64(total, _mm256_slli_epi64(count(eights), 3));
    total = _mm256_add_epi64(total, _mm256_slli_epi64(count(fours), 2));
    total = _mm256_add_epi64(total, _mm256_slli_epi64(count(twos), 1));
    total = _mm256_add_epi64(total, count(ones));
    (_mm256_extract_epi64(total, 0)
        + _mm256_extract_epi64(total, 1)
        + _mm256_extract_epi64(total, 2)
        + _mm256_extract_epi64(total, 3)) as u64
}

#[target_feature(enable = "avx2")]
pub unsafe fn distance_vect(x: &[u8], y: &[u8]) -> u64 {
    assert_eq!(x.len(), y.len());
    let mut accum = 0;
    let (x_head, x_mid, x_tail) = x.align_to::<__m256i>();
    let (y_head, y_mid, y_tail) = y.align_to::<__m256i>();
    // TODO: remove that with additional checks
    assert_eq!(x_head.len(), y_head.len());
    accum += super::distance_faster(x_head, y_head);
    let main_block_length = 16 * (x_mid.len() / 16);
    let x_ptr_avx = x_mid.as_ptr() as *const __m256i;
    let y_ptr_avx = y_mid.as_ptr() as *const __m256i;

    accum += avx2_harvey_seal_popcnt(x_ptr_avx, y_ptr_avx, main_block_length);

    let x_avx_tail = x_ptr_avx.add(main_block_length) as *const u8;
    let y_avx_tail = y_ptr_avx.add(main_block_length) as *const u8;

    let x_final = std::slice::from_raw_parts(
        x_avx_tail,
        32 * (x_mid.len() - main_block_length) + x_tail.len(),
    );

    let y_final = std::slice::from_raw_parts(
        y_avx_tail,
        32 * (y_mid.len() - main_block_length) + y_tail.len(),
    );

    accum += super::distance_faster(x_final, y_final);

    accum
}
