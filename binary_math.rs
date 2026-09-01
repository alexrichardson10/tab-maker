// there's about a thousand indexing bugs in here
// later me's problem

pub mod binary_math {
    fn fix_negative(a: &Vec<i8>, should_add: bool) -> Vec<u8> {
        let mut mod_val: Vec<i8> = vec![1i8; 256]; mod_val[255] = 0; mod_val[254] = 0; mod_val[251] = 0;
        let mut result: Vec<i8> = vec![0i8; if should_add { if a.len() > mod_val.len() { a.len() } else { mod_val.len() }} else { a.len() }];

        if should_add {
            if a.len() < mod_val.len() {
                for i in 0..(mod_val.len() - a.len()) {
                    result[i] = mod_val[i];
                }
                for i in (mod_val.len() - a.len())..mod_val.len() {
                    result[i] = mod_val[i] + a[i - (mod_val.len() - a.len())];
                }
            } else {
                panic!();
            }

            for i in (1..result.len()).rev() {
                if result[i] < 0 {
                    result[i] += 2;
                    result[i - 1] -= 1;
                }
                if result[i] > 1 {
                    result[i] -= 2;
                    result[i - 1] += 1;
                }
            }
        } else {
          for i in 0..result.len() {
              result[i] = a[i];
          }
        }

        let mut result_real: Vec<u8> = vec![0u8; result.len()];

        for i in 0..result.len() {
            result_real[i] = result[i] as u8;
        }

        result_real
    }

    pub fn binary_add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut result: Vec<u8> = vec![0u8; a.len() + 1];

        if a.len() >= b.len() {
            // for the shared bits
            for i in (1..b.len() + 1).rev() {
                //println!("{}", i);
                result[i + a.len() - b.len()] += a[(a.len() - b.len()) + i - 1] + b[i - 1];
            }
            // for each of the bits in a that b is missing
            for i in (1..a.len() - b.len() + 1).rev() {
                result[i] += a[i - 1];
            }
        } else {
            return binary_add(b, a);
        }

        // carry
        for i in (1..result.len()).rev() {
            if result[i] >= 2 {
                result[i] -= 2;
                result[i - 1] += 1;
            }
        }

        result
    }

    pub fn binary_sub(a1: &Vec<u8>, b1: &Vec<u8>) -> Vec<u8> {
        // makes sure the vecs are the same length
        // yes, this is horribly inefficient. No, I don't care.
        let zeroes_a: usize = if b1.len() > a1.len() { b1.len() - a1.len() } else { 0 };
        let mut a: Vec<u8> = vec![0u8; zeroes_a];
        for i in a1.iter() { a.push(*i); }
        let zeroes_b: usize = if a1.len() > b1.len() { a1.len() - b1.len() } else { 0 };
        let mut b: Vec<u8> = vec![0u8; zeroes_b];
        for i in b1.iter() { b.push(*i); }

        let mut result: Vec<i8> = vec![0i8; if a.len() > b.len() { a.len() } else { b.len() } + 1];

        for i in (0..result.len() - 1).rev() {
            result[i + 1] += a[i] as i8 - b[i] as i8;
            if result[i + 1] < 0 {
                result[i + 1] += 2;
                result[i] -= 1;
            }
        }

        let mut need_to_add: bool = false;
        for i in 0..result.len() {
            if result[i] < 0 {
                need_to_add = true;
            }
        }

        fix_negative(&result, need_to_add)
    }

    pub fn binary_mul(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut bits_after_mul: Vec<Vec<u8>> = vec![vec![0u8; a.len() + b.len()]; a.len()];
        let mut carry: Vec<u8> = vec![0u8; a.len() + b.len() + 1];

        // for every bit in a
        for i in 0..a.len() {
            let curr_vec = &mut bits_after_mul[i];

            // if the bit is not zero
            if a[i] != 0 {
                // for every bit in b, in reverse order
                for j in (0..b.len()).rev() {
                    // set the bit in the vector to the bit in the array
                    curr_vec[j + i + 1] = b[j];
                }
            }
        }

        let mut result: Vec<u8> = vec![0; a.len() + b.len() + 1];

        // for every vector in the multiplied bits
        for i in (0..bits_after_mul[0].len()).rev() {
            // for every set of bits in the multiplied bits
            for num in bits_after_mul.iter() {
                // if the bit is 1
                if num[i] == 1 {
                    // add it to the result
                    result[i] += 1;
                }
            }

            // add the carried value(s)
            result[i] += carry[i + 1];

            carry[i] = result[i] / 2;
            result[i + 1] = result[i] % 2;
        }

        result[1..].to_vec()
    }

    pub fn binary_div(a1: &Vec<u8>, b1: &Vec<u8>) -> Vec<u8> {
        // remove the leading zeroes... breaks things
        let a1= trim_leading_zeroes(a1.clone());
        let b1 = trim_leading_zeroes(b1.clone());

        let mut a: Vec<i8> = vec![0i8; a1.len()];
        for i in 0..a1.len() { a[i] = a1[i] as i8; } // copy the u8 into i8s... subtraction sucks
        let b: Vec<u8> = b1.clone();
        if a.len() < b.len() { return a1.to_vec(); };

        let mut result: Vec<u8> = vec![0u8; 0];
        let mut pos: u32 = 0;
        let mut pos2: u32 = 0;

        // for every sequence of bits that COULD contain the divisor
        for _i in b.len() - 1..a.len()  {
            let mut is_larger: bool = true;
            let mut skip: bool = false;

            for j in pos..pos2 {
                if a[j as usize] == 1 { skip = true;}
            }

            if !skip {
                // for each bit in the slice from pos to i
                for j in 0..b.len() {
                    // if it's bigger than the divisor (should never happen on bit 1)
                    if a[pos2 as usize + j] > b[j] as i8 {
                        break;
                    }  /* if it's equal to the divisor */ else if a[pos2 as usize + j] == b[j] as i8 {
                        continue;
                    } // if it's smaller
                    is_larger = false;
                    break;
                }
            }

            //println!("is_larger: {}", is_larger);

            // if the bits are >= the divisor
            if is_larger {
                // subtract b1, don't propagate
                for k in (0..b.len()).rev() {
                    a[k + pos2 as usize] -= b[k] as i8;
                }
                // propagate over anything that might be important
                for k in (pos as usize..pos2 as usize + b.len()).rev() {
                    if a[k] < 0 {
                        a[k] += 2;
                        a[k - 1] -= 1;
                    }
                }

                result.push(1);
                pos += 1;
                pos2 += 1;

                continue;
            }

            result.push(0);
            pos2 += 1;

        }

        result
    }

    pub fn binary_mod(num: &Vec<u8>) -> Vec<u8> {
        let mut mod_val: Vec<u8> = vec![1u8; 256]; mod_val[255] = 0; mod_val[254] = 0; mod_val[251] = 0;
        let times_fits: Vec<u8> = binary_div(&num, &mod_val);
        let val_to_remove: Vec<u8> = binary_mul(&times_fits, &mod_val);
        binary_sub(&num, &val_to_remove)
    }

    pub fn trim_leading_zeroes(num: Vec<u8>) -> Vec<u8> {
        let mut found_start: bool = false;
        let mut result: Vec<u8> = vec![];
        
        for i in num.iter() {
            if *i == 0 && found_start {
               result.push(0); 
            } else if *i == 1 && !found_start {
                found_start = true;
            }
            
            if *i == 1 {
                result.push(1);
            }
        }

        result
    }
}