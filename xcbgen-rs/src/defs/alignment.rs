/// An alignment specification.
///
/// This structure represents a requirement that some byte position `pos` satisfies
/// `pos % align == offset`.
///
/// `align` must be a power of 2 and `offset` must be less than `align`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Alignment {
    align: u32,
    offset: u32,
}

impl Alignment {
    /// Creates a new `Alignment` with `align` and `offset`.
    ///
    /// # Panics
    ///
    /// Panics if `align` is not a power of two or if `offset` is
    /// equal or greater than `align`.
    pub fn new(align: u32, offset: u32) -> Self {
        assert!(align.is_power_of_two() && offset < align);
        Self { align, offset }
    }

    /// Returns the value of `align`.
    #[inline]
    pub fn align(self) -> u32 {
        self.align
    }

    /// Returns the value of `offset`.
    #[inline]
    pub fn offset(self) -> u32 {
        self.offset
    }

    /// Advance this alignment specification by some variably sized object.
    ///
    /// The resulting value describes the alignment at the end of the variably sized object if it
    /// is aligned by `self` at its beginning.
    #[must_use]
    pub fn advance_variable_size(self, size: VariableSize) -> Self {
        let align = if size.incr == 0 {
            self.align
        } else {
            self.align.min(size.incr)
        };
        let offset = self.offset.wrapping_add(size.base) % align;
        Self { align, offset }
    }

    /// Returns an alignment that meets `self` and `other`.
    pub fn union(self, other: Self) -> Option<Self> {
        match self.align.cmp(&other.align) {
            std::cmp::Ordering::Less => {
                if (other.offset % self.align) != self.offset {
                    None
                } else {
                    Some(other)
                }
            }
            std::cmp::Ordering::Equal => {
                if self.offset != other.offset {
                    None
                } else {
                    Some(self)
                }
            }
            std::cmp::Ordering::Greater => {
                if (self.offset % other.align) != other.align {
                    None
                } else {
                    Some(self)
                }
            }
        }
    }

    /// Returns an alignment that is met by `self` and `other`.
    #[must_use]
    pub fn intersection(self, other: Self) -> Self {
        let align = self.align.min(other.align);
        let offset1 = self.offset % align;
        let offset2 = other.offset % align;

        if offset1 != offset2 {
            let align1 = (1u32 << offset1.trailing_zeros().min(31)).min(align);
            let align2 = (1u32 << offset2.trailing_zeros().min(31)).min(align);
            Self {
                align: align1.min(align2),
                offset: 0,
            }
        } else {
            Self {
                align,
                offset: offset1,
            }
        }
    }

    /// Returns whether `self` meets the alignment requirements
    /// of `required`.
    pub fn meets(self, required: Self) -> bool {
        // `self.align >= required.align` is equivalent to
        // `self.align % required.align == 0` because `align`
        // is always a power of 2.
        self.align >= required.align && (self.offset % required.align) == required.offset
    }
}

/// Represents the size of an object that has a variable size.
///
/// An object has a minimum size that is described by `base`. It can then have a variable number of
/// increments of size `incr`. That is, the object can have a size of `base + incr * n` for some
/// non-negative value of `n`.
///
/// `incr` must be zero or a power of 2
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableSize {
    base: u32,
    incr: u32,
}

impl VariableSize {
    /// Creates a new `VariableSize` with `base` and `incr`.
    ///
    /// # Panics
    ///
    /// Panics if `incr` neither zero nor a power of 2.
    pub fn new(base: u32, incr: u32) -> Self {
        assert!(incr == 0 || incr.is_power_of_two());
        Self { base, incr }
    }

    /// Returns the value of `base`.
    #[inline]
    pub fn base(self) -> u32 {
        self.base
    }

    /// Returns the value of `incr`.
    #[inline]
    pub fn incr(self) -> u32 {
        self.incr
    }

    /// Get the minimum of two values, but not zero (unless both are zero).
    fn incr_union(incr1: u32, incr2: u32) -> u32 {
        if incr1 == 0 {
            incr2
        } else if incr2 == 0 {
            incr1
        } else {
            incr1.min(incr2)
        }
    }

    /// Reduce the base by the given increment.
    fn reduce_base(base: u32, incr: u32) -> u32 {
        if incr == 0 {
            base
        } else {
            base % incr
        }
    }

    /// Create an instance that describes a zero-sized type.
    #[inline]
    pub fn zero() -> Self {
        Self { base: 0, incr: 0 }
    }

    /// Return a description of the size of things when `self` is appended to `other`.
    ///
    /// This function returns an over-approximation. This means that all sizes that can be created
    /// by such a concatenation are described by the returned object. However, not all sizes that
    /// are described by the returned object can necessarily be constructed by such a
    /// concatenation.
    #[must_use]
    pub fn append(self, other: Self) -> Self {
        Self {
            // FIXME: check overflow
            base: self.base + other.base,
            incr: Self::incr_union(self.incr, other.incr),
        }
    }

    /// Returns an `VariableSize` that can represent all the sizes
    /// represented by `self` and `other`.
    #[must_use]
    pub fn union(self, other: Self) -> Self {
        let incr_union = Self::incr_union(self.incr, other.incr);
        if self.base == other.base {
            Self {
                base: self.base,
                incr: incr_union,
            }
        } else {
            let base1 = Self::reduce_base(self.base, incr_union);
            let base2 = Self::reduce_base(other.base, incr_union);
            if base1 == base2 {
                Self {
                    base: base1,
                    incr: incr_union,
                }
            } else if base1 == 0 || base2 == 0 {
                // Compute base1.min(base2). This only works because both values are powers of two.
                let incr = 1u32 << (base1 | base2).trailing_zeros().min(31);
                Self { base: 0, incr }
            } else {
                let min_base = base1.min(base2);
                let max_base = base1.max(base2);
                let incr1 = 1u32 << min_base.trailing_zeros().min(31);
                let incr2 = 1u32 << (max_base - min_base).trailing_zeros().min(31);
                if incr1 > incr2 {
                    Self {
                        base: max_base - min_base,
                        incr: incr1,
                    }
                } else {
                    Self {
                        base: min_base,
                        incr: incr2,
                    }
                }
                /*Self {
                    base: 0,
                    incr: 1u32 << (base1 | base2).trailing_zeros().min(31),
                }*/
            }
        }
    }

    /// Describe the size of an arbitrary number of elements.
    #[must_use]
    pub fn zero_one_or_many(self) -> Self {
        // Self represents sizes `base + incr * n`, where `n >= 0`.
        // The returned value must represent sizes `(base + incr * n) * m`
        // (or a superset), where `m >= 0`.
        if self.base == 0 && self.incr == 0 {
            Self::zero()
        } else {
            // `(base + incr * n) * m = base * m + incr * n * m = gcd(base, incr) * l`
            Self {
                base: 0,
                // `self.base | self.incr` won't be zero, so `trailing_zeros < 32`.
                incr: 1u32 << (self.base | self.incr).trailing_zeros(),
            }
        }
    }

    /// Describe the size of `n` elements of this type.
    #[must_use]
    pub fn repeat_n(self, n: u32) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            let (base, base_overflow) = self
                .base
                .checked_mul(n)
                .map(|base| (base, false))
                .unwrap_or_else(|| Self::reduce_base(self.base, self.incr).overflowing_mul(n));

            let (mut incr, incr_overflow) = self.incr.overflowing_mul(n);
            if incr_overflow {
                incr = 1 << 31;
            }

            if base_overflow {
                incr = 1 << (incr | base | 1 << 31).trailing_zeros().min(31);
                Self { base: 0, incr }
            } else {
                if incr != 0 {
                    incr = 1 << incr.trailing_zeros().min(31);
                }
                Self { base, incr }
            }
        }
    }
}

/// Some computed alignment information.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComplexAlignment {
    /// Alignment at the beginning of the structure.
    pub begin: Alignment,
    /// Alignment at the end of the structure.
    pub body: AlignBody,
    /// The size of the largest alignment pad inside this object.
    ///
    /// At the time of writing, the code generator assumes that the start of an object is suitable
    /// aligned for the largest internal alignment. This property needs to be checked and for that
    /// this field exists.
    pub internal_align: u32,
}

/// Alignment information after some structure.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlignBody {
    /// Some `VariableSize` can describe the size of the body.
    Size(VariableSize),

    /// Information about the alignment after the structure.
    EndAlign(Alignment),
}

impl ComplexAlignment {
    /// Create alignment information for some structure with a fixed size `size` that needs to be
    /// aligned to a multiple of `align`.
    #[inline]
    pub fn fixed_size(size: u32, align: u32) -> Self {
        Self {
            begin: Alignment { align, offset: 0 },
            body: AlignBody::Size(VariableSize::new(size, 0)),
            internal_align: 1,
        }
    }

    /// Create alignment information for an empty structure.
    #[inline]
    pub fn zero_sized() -> Self {
        Self::fixed_size(0, 1)
    }

    /// Get the alignment after the structure.
    pub fn end_align(self) -> Alignment {
        match self.body {
            AlignBody::Size(size) => self.begin.advance_variable_size(size),
            AlignBody::EndAlign(end_align) => end_align,
        }
    }

    /// Compute alignment information for an arbitrary repetition of the current description.
    ///
    /// This returns `None` if the alignment of this structure is violated, i.e. it cannot be
    /// repeated.
    pub fn zero_one_or_many(self) -> Option<Self> {
        if !self.end_align().meets(self.begin) {
            None
        } else {
            match self.body {
                AlignBody::Size(size) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::Size(size.zero_one_or_many()),
                    internal_align: self.internal_align,
                }),
                AlignBody::EndAlign(end_align) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::EndAlign(self.begin.union(end_align)?),
                    internal_align: self.internal_align,
                }),
            }
        }
    }

    /// Compute alignment information for `n` repetitions of this type.
    ///
    /// This returns `None` if the alignment of this structure is violated, i.e. it cannot be
    /// repeated.
    pub fn repeat_n(self, n: u32) -> Option<Self> {
        if n == 0 {
            Some(Self {
                begin: self.begin,
                body: AlignBody::Size(VariableSize::zero()),
                internal_align: self.internal_align,
            })
        } else if n > 1 && !self.end_align().meets(self.begin) {
            None
        } else {
            match self.body {
                AlignBody::Size(size) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::Size(size.repeat_n(n)),
                    internal_align: self.internal_align,
                }),
                AlignBody::EndAlign(end_align) => Some(Self {
                    begin: self.begin,
                    body: AlignBody::EndAlign(end_align),
                    internal_align: self.internal_align,
                }),
            }
        }
    }

    /// Get a new alignment description for the combination of two types right after each other.
    pub fn append(self, next: Self) -> Option<Self> {
        if !self.end_align().meets(next.begin) {
            None
        } else {
            let new_body = match (self.body, next.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.append(next_size))
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(curr_end_align.advance_variable_size(next_size))
                }
                (_, AlignBody::EndAlign(next_end_align)) => AlignBody::EndAlign(next_end_align),
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(next.internal_align),
            })
        }
    }

    /// Compute a new alignment description for two types appearing in a union, i.e. both starting
    /// at the same position.
    pub fn union_append(self, other: Self) -> Option<Self> {
        if !self.begin.meets(other.begin) {
            None
        } else {
            let new_body = match (self.body, other.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.union(next_size))
                }
                (AlignBody::Size(curr_size), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(
                        self.begin
                            .advance_variable_size(curr_size)
                            .intersection(next_end_align),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(
                        curr_end_align.intersection(self.begin.advance_variable_size(next_size)),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(curr_end_align.intersection(next_end_align))
                }
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(other.internal_align),
            })
        }
    }

    /// Compute alignment information of types that can appear optionally after each other.
    ///
    /// This computes the alignment for the concatenation of bitcases.
    pub fn bitcase_append(self, next: Self) -> Option<Self> {
        if !self.end_align().meets(next.begin) {
            None
        } else {
            let new_body = match (self.body, next.body) {
                (AlignBody::Size(curr_size), AlignBody::Size(next_size)) => {
                    AlignBody::Size(curr_size.union(curr_size.append(next_size)))
                }
                (AlignBody::Size(curr_size), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(
                        self.begin
                            .advance_variable_size(curr_size)
                            .intersection(next_end_align),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::Size(next_size)) => {
                    AlignBody::EndAlign(
                        curr_end_align
                            .intersection(curr_end_align.advance_variable_size(next_size)),
                    )
                }
                (AlignBody::EndAlign(curr_end_align), AlignBody::EndAlign(next_end_align)) => {
                    AlignBody::EndAlign(curr_end_align.intersection(next_end_align))
                }
            };
            Some(Self {
                begin: self.begin,
                body: new_body,
                internal_align: self.internal_align.max(next.internal_align),
            })
        }
    }
}
