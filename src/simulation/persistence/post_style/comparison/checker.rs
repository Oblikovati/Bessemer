use super::super::legend_comparison::scalar_differs;
use super::state::STYLE_CHANNELS;
use super::{
    SimulationStyleComparisonCheck as Check, SimulationStyleComparisonColor as Color,
    SimulationStyleComparisonPlan, SimulationStyleComparisonScalar as Scalar,
    SimulationStyleComparisonState, SimulationStyleComparisonWord as Word,
};
use crate::{BessemerError, FailureClass, Result};

struct StyleDependencyChecks<'a> {
    left: &'a SimulationStyleComparisonState,
    right: &'a SimulationStyleComparisonState,
    checks: Vec<Check>,
}

impl SimulationStyleComparisonState {
    /// Compare all reached native dependencies under explicit SSE assumptions; e.g. `left.compare_masked_exceptions_daz_off(&right)?.outcome()`.
    pub fn compare_masked_exceptions_daz_off(
        &self,
        other: &Self,
    ) -> Result<SimulationStyleComparisonPlan> {
        let mut scope = StyleDependencyChecks {
            left: self,
            right: other,
            checks: Vec::new(),
        };
        let first = scope.first_difference()?;
        Ok(SimulationStyleComparisonPlan::complete(
            self.clone(),
            other.clone(),
            scope.checks,
            first,
        ))
    }
}

impl StyleDependencyChecks<'_> {
    fn first_difference(&mut self) -> Result<Option<Check>> {
        if let Some(first) = self.mapping_difference()? {
            return Ok(Some(first));
        }
        if let Some(first) = self.tessellation_difference()? {
            return Ok(Some(first));
        }
        if self.legend_differs()? {
            return Ok(Some(Check::Legend));
        }
        for member in [Word::Word28C, Word::Word290, Word::Word294, Word::Word298] {
            if self.word_differs(member, true)? {
                return Ok(Some(Check::Word(member)));
            }
        }
        if self.color_differs(Color::Color29C)? {
            return Ok(Some(Check::Color(Color::Color29C)));
        }
        self.hidden_difference()
    }

    fn mapping_difference(&mut self) -> Result<Option<Check>> {
        if self.word_differs(Word::Word1B8, false)? {
            return Ok(Some(Check::Word(Word::Word1B8)));
        }
        if matches!(self.left.word_bits(Word::Word1B8), Some(1 | 2))
            && self.color_differs(Color::Color1BC)?
        {
            return Ok(Some(Check::Color(Color::Color1BC)));
        }
        if self.left.word_bits(Word::Word1B8) == Some(2) && self.color_differs(Color::Color1CC)? {
            return Ok(Some(Check::Color(Color::Color1CC)));
        }
        Ok(None)
    }

    fn tessellation_difference(&mut self) -> Result<Option<Check>> {
        if self.word_differs(Word::Word21C, false)? {
            return Ok(Some(Check::Word(Word::Word21C)));
        }
        if self.left.word_bits(Word::Word21C) == Some(0) {
            return Ok(None);
        }
        if self.scalar_differs(Scalar::Alpha22C)? {
            return Ok(Some(Check::Scalar(Scalar::Alpha22C)));
        }
        if self.word_differs(Word::Word230, false)? {
            return Ok(Some(Check::Word(Word::Word230)));
        }
        if self.left.word_bits(Word::Word230) == Some(0) && self.color_differs(Color::Color220)? {
            return Ok(Some(Check::Color(Color::Color220)));
        }
        Ok(None)
    }

    fn hidden_difference(&mut self) -> Result<Option<Check>> {
        if self.word_differs(Word::Word1EC, false)? {
            return Ok(Some(Check::Word(Word::Word1EC)));
        }
        if self.left.word_bits(Word::Word1EC) != Some(0) {
            if self.word_differs(Word::Word1F0, true)? {
                return Ok(Some(Check::Word(Word::Word1F0)));
            }
            if self.color_differs(Color::Color1F4)? {
                return Ok(Some(Check::Color(Color::Color1F4)));
            }
        }
        self.custom_difference()
    }

    fn custom_difference(&mut self) -> Result<Option<Check>> {
        if self.word_differs(Word::Word204, false)? {
            return Ok(Some(Check::Word(Word::Word204)));
        }
        if self.left.word_bits(Word::Word204) != Some(0) {
            if self.word_differs(Word::Word208, true)? {
                return Ok(Some(Check::Word(Word::Word208)));
            }
            if self.color_differs(Color::Color20C)? {
                return Ok(Some(Check::Color(Color::Color20C)));
            }
        }
        if self.scalar_differs(Scalar::Alpha2B8)? {
            return Ok(Some(Check::Scalar(Scalar::Alpha2B8)));
        }
        Ok(None)
    }

    fn word_differs(&mut self, member: Word, right_first: bool) -> Result<bool> {
        let read_left =
            || require_style_bits(self.left.word_bits(member), "left", member.field_path());
        let read_right =
            || require_style_bits(self.right.word_bits(member), "right", member.field_path());
        let (left, right) = if right_first {
            let right = read_right()?;
            (read_left()?, right)
        } else {
            let left = read_left()?;
            (left, read_right()?)
        };
        self.checks.push(Check::Word(member));
        Ok(left != right)
    }

    fn scalar_differs(&mut self, member: Scalar) -> Result<bool> {
        let path = format!("{}.component_3", member.color().field_path());
        let right = require_style_bits(self.right.scalar_bits(member), "right", &path)?;
        let left = require_style_bits(self.left.scalar_bits(member), "left", &path)?;
        self.checks.push(Check::Scalar(member));
        Ok(scalar_differs(left, right))
    }

    fn color_differs(&mut self, color: Color) -> Result<bool> {
        for channel in STYLE_CHANNELS {
            let path = format!("{}.component_{}", color.field_path(), channel as usize);
            let right =
                require_style_bits(self.right.component_bits(color, channel), "right", &path)?;
            let left = require_style_bits(self.left.component_bits(color, channel), "left", &path)?;
            if scalar_differs(left, right) {
                self.checks.push(Check::Color(color));
                return Ok(true);
            }
        }
        self.checks.push(Check::Color(color));
        Ok(false)
    }

    fn legend_differs(&mut self) -> Result<bool> {
        let result = self
            .left
            .legend
            .different_masked_exceptions_daz_off(&self.right.legend)
            .map_err(|error| {
                BessemerError::new(
                    "conditional PostStyle comparison",
                    &error.entity,
                    error.class,
                    format!("{}: {}", error.operation, error.cause),
                )
            })?;
        self.checks.push(Check::Legend);
        Ok(result)
    }
}

fn require_style_bits(bits: Option<u32>, side: &str, path: &str) -> Result<u32> {
    bits.ok_or_else(|| BessemerError::new("conditional PostStyle comparison", &format!("{side}.{path}"),
        FailureClass::Missing, "None; expected Some(explicit loaded raw32 input); no initializer or migration default inferred"))
}
