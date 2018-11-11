//! Tier 2 features.
//!
//! These are the features included only in ADS1x14, ADS1x15

use { Ads1x1x, Error, interface, ic, ComparatorMode, ComparatorPolarity,
      ComparatorLatching, Register, BitFlags, conversion };

impl<DI, IC, CONV, MODE, E> Ads1x1x<DI, IC, CONV, MODE>
where
    DI: interface::WriteData<Error = E>,
    IC: ic::Tier2Features,
    CONV: conversion::ConvertThreshold<E>
{
    /// Set comparator lower threshold
    pub fn set_low_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::LOW_TH, register_value)
    }

    /// Set comparator upper threshold
    pub fn set_high_threshold(&mut self, value: i16) -> Result<(), Error<E>> {
        let register_value = CONV::convert_threshold(value)?;
        self.iface.write_register(Register::HIGH_TH, register_value)
    }

    /// Set comparator mode
    pub fn set_comparator_mode(&mut self, mode: ComparatorMode) -> Result<(), Error<E>> {
        let config;
        match mode {
            ComparatorMode::Traditional => config = self.config.with_low(BitFlags::COMP_MODE),
            ComparatorMode::Window      => config = self.config.with_high(BitFlags::COMP_MODE)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator polarity
    pub fn set_comparator_polarity(&mut self, polarity: ComparatorPolarity) -> Result<(), Error<E>> {
        let config;
        match polarity {
            ComparatorPolarity::ActiveLow  => config = self.config.with_low( BitFlags::COMP_POL),
            ComparatorPolarity::ActiveHigh => config = self.config.with_high(BitFlags::COMP_POL)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }

    /// Set comparator latching
    pub fn set_comparator_latching(&mut self, latching: ComparatorLatching) -> Result<(), Error<E>> {
        let config;
        match latching {
            ComparatorLatching::Nonlatching => config = self.config.with_low( BitFlags::COMP_LAT),
            ComparatorLatching::Latching    => config = self.config.with_high(BitFlags::COMP_LAT)
        }
        self.iface.write_register(Register::CONFIG, config.bits)?;
        self.config = config;
        Ok(())
    }
}
