#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `SEL` reader - Select the phase for the Output clock"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - Select the phase for the Output clock"]
pub type SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UNIT` reader - Delay Defines the delay of a Unit delay cell"]
pub type UNIT_R = crate::FieldReader;
#[doc = "Field `UNIT` writer - Delay Defines the delay of a Unit delay cell"]
pub type UNIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `LNG` reader - Delay line length value"]
pub type LNG_R = crate::FieldReader<u16>;
#[doc = "Field `LNG` writer - Delay line length value"]
pub type LNG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `LNGF` reader - Length valid flag"]
pub type LNGF_R = crate::BitReader;
#[doc = "Field `LNGF` writer - Length valid flag"]
pub type LNGF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Select the phase for the Output clock"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Delay Defines the delay of a Unit delay cell"]
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - Delay line length value"]
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Length valid flag"]
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select the phase for the Output clock"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<CFGR_SPEC, 0> {
        SEL_W::new(self)
    }
    #[doc = "Bits 8:14 - Delay Defines the delay of a Unit delay cell"]
    #[inline(always)]
    #[must_use]
    pub fn unit(&mut self) -> UNIT_W<CFGR_SPEC, 8> {
        UNIT_W::new(self)
    }
    #[doc = "Bits 16:27 - Delay line length value"]
    #[inline(always)]
    #[must_use]
    pub fn lng(&mut self) -> LNG_W<CFGR_SPEC, 16> {
        LNG_W::new(self)
    }
    #[doc = "Bit 31 - Length valid flag"]
    #[inline(always)]
    #[must_use]
    pub fn lngf(&mut self) -> LNGF_W<CFGR_SPEC, 31> {
        LNGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DLYB configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
