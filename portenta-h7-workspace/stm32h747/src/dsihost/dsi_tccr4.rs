#[doc = "Register `DSI_TCCR4` reader"]
pub type R = crate::R<DSI_TCCR4_SPEC>;
#[doc = "Register `DSI_TCCR4` writer"]
pub type W = crate::W<DSI_TCCR4_SPEC>;
#[doc = "Field `LPWR_TOCNT` reader - LPWR_TOCNT"]
pub type LPWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `LPWR_TOCNT` writer - LPWR_TOCNT"]
pub type LPWR_TOCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LPWR_TOCNT_R {
        LPWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn lpwr_tocnt(&mut self) -> LPWR_TOCNT_W<DSI_TCCR4_SPEC, 0> {
        LPWR_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR4_SPEC;
impl crate::RegisterSpec for DSI_TCCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr4::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr4::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_TCCR4 to value 0"]
impl crate::Resettable for DSI_TCCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
