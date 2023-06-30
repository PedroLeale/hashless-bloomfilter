use base_58_dictionary::from_base58_dictionary;
use bit_vec::BitVec;
use std::cmp;
mod base_58_dictionary;

#[derive(Debug)]
pub struct BloomFilter {
    bitvec: BitVec,
    num_hashes: usize, //not truly hashes
    number_of_bits: usize,
    items_count: usize,
}

#[allow(dead_code)]
impl BloomFilter {
    pub fn new_fp_rate(items_count: usize, fp_rate: f64) -> BloomFilter {
        let num_bits: usize = Self::calc_num_bits(items_count, fp_rate);
        let num_hashes: usize = Self::calc_num_hashes(num_bits, items_count);

        BloomFilter {
            bitvec: BitVec::from_elem(num_bits, false),
            num_hashes: num_hashes,
            number_of_bits: num_bits,
            items_count: 0,
        }
    }

    pub fn insert(&mut self, value: &str) {
        let pos: Vec<usize> = BloomFilter::bit_offset(
            &from_base58_dictionary(&value[1..self.num_hashes]),
            self.number_of_bits,
            self.num_hashes,
        );

        for i in 1..self.num_hashes {
            self.bitvec.set(pos[i], true);
        }

        self.items_count = self.items_count + 1;
    }

    pub fn contains(&self, value: &str) -> bool { //True for maybe it contains
        let pos: Vec<usize> = BloomFilter::bit_offset( //False for sure it doesn't contain
            &from_base58_dictionary(&value[1..self.num_hashes]),
            self.number_of_bits,
            self.num_hashes,
        );

        for i in 0..self.num_hashes {
            if self.bitvec.get(pos[i]).unwrap() == false {
                return false;
            }
        }
        true
    }

    fn bit_offset(encoded_string: &Vec<u8>, max_size: usize, num: usize) -> Vec<usize> {
        let mut result: usize = 0;
        let mut base_potency: usize = 1;
        let mut offset: Vec<usize> = Vec::new();

        for i in 0..num { //Better precision with 2 * num, but lower performance
            result += encoded_string[(i) % encoded_string.len()] as usize * base_potency;
            base_potency *= 58;
            offset.push(result % max_size);
        }

        offset
    }

    pub fn false_positions(&self) -> usize {
        let mut false_pos = 0;
        for i in 0..self.bitvec.len() {
            if !self.bitvec[i] {
                false_pos += 1;
            }
        }
        false_pos
    }

    pub fn true_positions(&self) -> usize {
        let mut true_pos = 0;
        for i in 0..self.bitvec.len() {
            if self.bitvec[i] {
                true_pos += 1;
            }
        }
        true_pos
    }

    pub fn get_num_bits(&self) -> usize {
        self.number_of_bits
    }

    pub fn get_num_hashes(&self) -> usize {
        self.num_hashes
    }

    fn calc_num_bits(items_count: usize, fp_rate: f64) -> usize {
        ((items_count as f64 * fp_rate.ln()) / (1.0f64 / 2.0f64.powf(2.0f64.ln())).ln())
            .ceil() as usize
    }

    fn calc_num_hashes(num_bits: usize, items_count: usize) -> usize {
        cmp::max(
            1,
            ((num_bits as f64 / items_count as f64) * 2.0f64.ln()).ceil() as usize,
        )
    }

    pub fn bitmap(&self) -> Vec<bool> {
        let mut bitmap = Vec::new();
        for i in 0..self.bitvec.len() {
            bitmap.push(self.bitvec[i]);
        }
        bitmap
    }

}