#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PMCR_SPEC>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PMCR_SPEC>;
#[doc = "Field `I2C1FMP` reader - I2C1 Fm+"]
pub type I2C1FMP_R = crate::BitReader;
#[doc = "Field `I2C1FMP` writer - I2C1 Fm+"]
pub type I2C1FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2FMP` reader - I2C2 Fm+"]
pub type I2C2FMP_R = crate::BitReader;
#[doc = "Field `I2C2FMP` writer - I2C2 Fm+"]
pub type I2C2FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3FMP` reader - I2C3 Fm+"]
pub type I2C3FMP_R = crate::BitReader;
#[doc = "Field `I2C3FMP` writer - I2C3 Fm+"]
pub type I2C3FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C4FMP` reader - I2C4 Fm+"]
pub type I2C4FMP_R = crate::BitReader;
#[doc = "Field `I2C4FMP` writer - I2C4 Fm+"]
pub type I2C4FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB6FMP` reader - PB(6) Fm+"]
pub type PB6FMP_R = crate::BitReader;
#[doc = "Field `PB6FMP` writer - PB(6) Fm+"]
pub type PB6FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB7FMP` reader - PB(7) Fast Mode Plus"]
pub type PB7FMP_R = crate::BitReader;
#[doc = "Field `PB7FMP` writer - PB(7) Fast Mode Plus"]
pub type PB7FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB8FMP` reader - PB(8) Fast Mode Plus"]
pub type PB8FMP_R = crate::BitReader;
#[doc = "Field `PB8FMP` writer - PB(8) Fast Mode Plus"]
pub type PB8FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PB9FMP` reader - PB(9) Fm+"]
pub type PB9FMP_R = crate::BitReader;
#[doc = "Field `PB9FMP` writer - PB(9) Fm+"]
pub type PB9FMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOSTE` reader - Booster Enable"]
pub type BOOSTE_R = crate::BitReader;
#[doc = "Field `BOOSTE` writer - Booster Enable"]
pub type BOOSTE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BOOSTVDDSEL` reader - Analog switch supply voltage selection"]
pub type BOOSTVDDSEL_R = crate::BitReader;
#[doc = "Field `BOOSTVDDSEL` writer - Analog switch supply voltage selection"]
pub type BOOSTVDDSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPIS` reader - Ethernet PHY Interface Selection"]
pub type EPIS_R = crate::FieldReader;
#[doc = "Field `EPIS` writer - Ethernet PHY Interface Selection"]
pub type EPIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PA0SO` reader - PA0 Switch Open"]
pub type PA0SO_R = crate::BitReader;
#[doc = "Field `PA0SO` writer - PA0 Switch Open"]
pub type PA0SO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PA1SO` reader - PA1 Switch Open"]
pub type PA1SO_R = crate::BitReader;
#[doc = "Field `PA1SO` writer - PA1 Switch Open"]
pub type PA1SO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC2SO` reader - PC2 Switch Open"]
pub type PC2SO_R = crate::BitReader;
#[doc = "Field `PC2SO` writer - PC2 Switch Open"]
pub type PC2SO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC3SO` reader - PC3 Switch Open"]
pub type PC3SO_R = crate::BitReader;
#[doc = "Field `PC3SO` writer - PC3 Switch Open"]
pub type PC3SO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2C1FMP_R {
        I2C1FMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2C2FMP_R {
        I2C2FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2C3FMP_R {
        I2C3FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2C4FMP_R {
        I2C4FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&self) -> PB6FMP_R {
        PB6FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&self) -> PB7FMP_R {
        PB7FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&self) -> PB8FMP_R {
        PB8FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&self) -> PB9FMP_R {
        PB9FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&self) -> EPIS_R {
        EPIS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&self) -> PA0SO_R {
        PA0SO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&self) -> PA1SO_R {
        PA1SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&self) -> PC2SO_R {
        PC2SO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&self) -> PC3SO_R {
        PC3SO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1fmp(&mut self) -> I2C1FMP_W<PMCR_SPEC, 0> {
        I2C1FMP_W::new(self)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2fmp(&mut self) -> I2C2FMP_W<PMCR_SPEC, 1> {
        I2C2FMP_W::new(self)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3fmp(&mut self) -> I2C3FMP_W<PMCR_SPEC, 2> {
        I2C3FMP_W::new(self)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4fmp(&mut self) -> I2C4FMP_W<PMCR_SPEC, 3> {
        I2C4FMP_W::new(self)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn pb6fmp(&mut self) -> PB6FMP_W<PMCR_SPEC, 4> {
        PB6FMP_W::new(self)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn pb7fmp(&mut self) -> PB7FMP_W<PMCR_SPEC, 5> {
        PB7FMP_W::new(self)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn pb8fmp(&mut self) -> PB8FMP_W<PMCR_SPEC, 6> {
        PB8FMP_W::new(self)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    #[must_use]
    pub fn pb9fmp(&mut self) -> PB9FMP_W<PMCR_SPEC, 7> {
        PB9FMP_W::new(self)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    #[must_use]
    pub fn booste(&mut self) -> BOOSTE_W<PMCR_SPEC, 8> {
        BOOSTE_W::new(self)
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    #[must_use]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<PMCR_SPEC, 9> {
        BOOSTVDDSEL_W::new(self)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    #[must_use]
    pub fn epis(&mut self) -> EPIS_W<PMCR_SPEC, 21> {
        EPIS_W::new(self)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    #[must_use]
    pub fn pa0so(&mut self) -> PA0SO_W<PMCR_SPEC, 24> {
        PA0SO_W::new(self)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    #[must_use]
    pub fn pa1so(&mut self) -> PA1SO_W<PMCR_SPEC, 25> {
        PA1SO_W::new(self)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    #[must_use]
    pub fn pc2so(&mut self) -> PC2SO_W<PMCR_SPEC, 26> {
        PC2SO_W::new(self)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    #[must_use]
    pub fn pc3so(&mut self) -> PC3SO_W<PMCR_SPEC, 27> {
        PC3SO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCR to value 0"]
impl crate::Resettable for PMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}