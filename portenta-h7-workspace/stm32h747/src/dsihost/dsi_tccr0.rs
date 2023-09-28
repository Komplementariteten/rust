#[doc = "Register `DSI_TCCR0` reader"]
pub type R = crate::R<DSI_TCCR0_SPEC>;
#[doc = "Register `DSI_TCCR0` writer"]
pub type W = crate::W<DSI_TCCR0_SPEC>;
#[doc = "Field `LPRX_TOCNT` reader - LPRX_TOCNT"]
pub type LPRX_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LPRX_TOCNT` writer - LPRX_TOCNT"]
pub type LPRX_TOCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `HSTX_TOCNT` reader - HSTX_TOCNT"]
pub type HSTX_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSTX_TOCNT` writer - HSTX_TOCNT"]
pub type HSTX_TOCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - LPRX_TOCNT"]
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - HSTX_TOCNT"]
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPRX_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W<DSI_TCCR0_SPEC, 0> {
        LPRX_TOCNT_W::new(self)
    }
    #[doc = "Bits 16:31 - HSTX_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W<DSI_TCCR0_SPEC, 16> {
        HSTX_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR0_SPEC;
impl crate::RegisterSpec for DSI_TCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr0::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr0::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_TCCR0 to value 0"]
impl crate::Resettable for DSI_TCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
