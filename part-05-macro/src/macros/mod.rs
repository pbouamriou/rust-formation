macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + crate::macros::count_exprs!($($tail),*));
}

pub(crate) use count_exprs;

macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {
        {
            use std::ops::Index;
            const MEM_SIZE: usize = crate::macros::count_exprs!($($inits),+);

            #[derive(Debug)]
            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            // let n = self.pos; // Error : Macro is not hygienic
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            // let a = IndexOffset { slice: &self.mem, offset: $ind }; // Error : Macro is not hygienic
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

pub(crate) use recurrence;

#[cfg(test)]
mod tests {
    #[test]
    fn macro_count_exprs() {
        const COUNT_0: u64 = count_exprs!();
        assert_eq!(COUNT_0, 0, "should be 0");

        const COUNT_1: u64 = count_exprs!(10 + 15);
        assert_eq!(COUNT_1, 1, "should be 1");

        const COUNT_2: u64 = count_exprs!(10 + 15, 12);
        assert_eq!(COUNT_2, 2, "should be 2");

        const COUNT_3: u64 = count_exprs!(10 + 15, 12, 0);
        assert_eq!(COUNT_3, 3, "should be 3");
    }

    #[test]
    fn macro_reccurrence() {
        let fib = recurrence!( a[n]: u64 = 0, 1; ...; a[n-2] + a[n-1]);
        assert_eq!(
            fib.take(8).collect::<Vec<u64>>(),
            [0, 1, 1, 2, 3, 5, 8, 13],
            "fib should be = 0 1 1 2 3 5 8 13"
        );
    }
}
