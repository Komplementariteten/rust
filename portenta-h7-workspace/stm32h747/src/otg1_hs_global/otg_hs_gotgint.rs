#[doc = "Register `OTG_HS_GOTGINT` reader"]
pub type R = crate::R<OTG_HS_GOTGINT_SPEC>;
#[doc = "Register `OTG_HS_GOTGINT` writer"]
pub type W = crate::W<OTG_HS_GOTGINT_SPEC>;
#[doc = "Field `SEDET` reader - Session end detected"]
pub type SEDET_R = crate::BitReader;
#[doc = "Field `SEDET` writer - Session end detected"]
pub type SEDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRSSCHG` reader - Session request success status change"]
pub type SRSSCHG_R = crate::BitReader;
#[doc = "Field `SRSSCHG` writer - Session request success status change"]
pub type SRSSCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNSSCHG` reader - Host negotiation success status change"]
pub type HNSSCHG_R = crate::BitReader;
#[doc = "Field `HNSSCHG` writer - Host negotiation success status change"]
pub type HNSSCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNGDET` reader - Host negotiation detected"]
pub type HNGDET_R = crate::BitReader;
#[doc = "Field `HNGDET` writer - Host negotiation detected"]
pub type HNGDET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADTOCHG` reader - A-device timeout change"]
pub type ADTOCHG_R = crate::BitReader;
#[doc = "Field `ADTOCHG` writer - A-device timeout change"]
pub type ADTOCHG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DBCDNE` reader - Debounce done"]
pub type DBCDNE_R = crate::BitReader;
#[doc = "Field `DBCDNE` writer - Debounce done"]
pub type DBCDNE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDCHNG` reader - ID input pin changed"]
pub type IDCHNG_R = crate::BitReader;
#[doc = "Field `IDCHNG` writer - ID input pin changed"]
pub type IDCHNG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&self) -> SEDET_R {
        SEDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&self) -> SRSSCHG_R {
        SRSSCHG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HNSSCHG_R {
        HNSSCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&self) -> HNGDET_R {
        HNGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&self) -> ADTOCHG_R {
        ADTOCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DBCDNE_R {
        DBCDNE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ID input pin changed"]
    #[inline(always)]
    pub fn idchng(&self) -> IDCHNG_R {
        IDCHNG_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SEDET_W<OTG_HS_GOTGINT_SPEC, 2> {
        SEDET_W::new(self)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SRSSCHG_W<OTG_HS_GOTGINT_SPEC, 8> {
        SRSSCHG_W::new(self)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HNSSCHG_W<OTG_HS_GOTGINT_SPEC, 9> {
        HNSSCHG_W::new(self)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HNGDET_W<OTG_HS_GOTGINT_SPEC, 17> {
        HNGDET_W::new(self)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> ADTOCHG_W<OTG_HS_GOTGINT_SPEC, 18> {
        ADTOCHG_W::new(self)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DBCDNE_W<OTG_HS_GOTGINT_SPEC, 19> {
        DBCDNE_W::new(self)
    }
    #[doc = "Bit 20 - ID input pin changed"]
    #[inline(always)]
    #[must_use]
    pub fn idchng(&mut self) -> IDCHNG_W<OTG_HS_GOTGINT_SPEC, 20> {
        IDCHNG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_GOTGINT_SPEC;
impl crate::RegisterSpec for OTG_HS_GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_gotgint::R`](R) reader structure"]
impl crate::Readable for OTG_HS_GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_gotgint::W`](W) writer structure"]
impl crate::Writable for OTG_HS_GOTGINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_GOTGINT to value 0"]
impl crate::Resettable for OTG_HS_GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
