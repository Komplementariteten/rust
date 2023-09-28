#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `CKSEL` reader - Clock selector"]
pub type CKSEL_R = crate::BitReader;
#[doc = "Field `CKSEL` writer - Clock selector"]
pub type CKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKPOL` reader - Clock Polarity"]
pub type CKPOL_R = crate::FieldReader;
#[doc = "Field `CKPOL` writer - Clock Polarity"]
pub type CKPOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock"]
pub type CKFLT_R = crate::FieldReader;
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock"]
pub type CKFLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger"]
pub type TRGFLT_R = crate::FieldReader;
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger"]
pub type TRGFLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PRESC` reader - Clock prescaler"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - Clock prescaler"]
pub type PRESC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TRIGSEL` reader - Trigger selector"]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - Trigger selector"]
pub type TRIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity"]
pub type TRIGEN_R = crate::FieldReader;
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity"]
pub type TRIGEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TIMOUT` reader - Timeout enable"]
pub type TIMOUT_R = crate::BitReader;
#[doc = "Field `TIMOUT` writer - Timeout enable"]
pub type TIMOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAVE` reader - Waveform shape"]
pub type WAVE_R = crate::BitReader;
#[doc = "Field `WAVE` writer - Waveform shape"]
pub type WAVE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAVPOL` reader - Waveform shape polarity"]
pub type WAVPOL_R = crate::BitReader;
#[doc = "Field `WAVPOL` writer - Waveform shape polarity"]
pub type WAVPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRELOAD` reader - Registers update mode"]
pub type PRELOAD_R = crate::BitReader;
#[doc = "Field `PRELOAD` writer - Registers update mode"]
pub type PRELOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COUNTMODE` reader - counter mode enabled"]
pub type COUNTMODE_R = crate::BitReader;
#[doc = "Field `COUNTMODE` writer - counter mode enabled"]
pub type COUNTMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENC` reader - Encoder mode enable"]
pub type ENC_R = crate::BitReader;
#[doc = "Field `ENC` writer - Encoder mode enable"]
pub type ENC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CFGR_SPEC, 0> {
        CKSEL_W::new(self)
    }
    #[doc = "Bits 1:2 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<CFGR_SPEC, 1> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<CFGR_SPEC, 3> {
        CKFLT_W::new(self)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<CFGR_SPEC, 6> {
        TRGFLT_W::new(self)
    }
    #[doc = "Bits 9:11 - Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CFGR_SPEC, 9> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 13:15 - Trigger selector"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<CFGR_SPEC, 13> {
        TRIGSEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<CFGR_SPEC, 17> {
        TRIGEN_W::new(self)
    }
    #[doc = "Bit 19 - Timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<CFGR_SPEC, 19> {
        TIMOUT_W::new(self)
    }
    #[doc = "Bit 20 - Waveform shape"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<CFGR_SPEC, 20> {
        WAVE_W::new(self)
    }
    #[doc = "Bit 21 - Waveform shape polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<CFGR_SPEC, 21> {
        WAVPOL_W::new(self)
    }
    #[doc = "Bit 22 - Registers update mode"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<CFGR_SPEC, 22> {
        PRELOAD_W::new(self)
    }
    #[doc = "Bit 23 - counter mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<CFGR_SPEC, 23> {
        COUNTMODE_W::new(self)
    }
    #[doc = "Bit 24 - Encoder mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<CFGR_SPEC, 24> {
        ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
