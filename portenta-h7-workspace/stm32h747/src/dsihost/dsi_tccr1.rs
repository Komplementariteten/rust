#[doc = "Register `DSI_TCCR1` reader"]
pub type R = crate::R<DSI_TCCR1_SPEC>;
#[doc = "Register `DSI_TCCR1` writer"]
pub type W = crate::W<DSI_TCCR1_SPEC>;
#[doc = "Field `HSRD_TOCNT` reader - HSRD_TOCNT"]
pub type HSRD_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSRD_TOCNT` writer - HSRD_TOCNT"]
pub type HSRD_TOCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - HSRD_TOCNT"]
    #[inline(always)]
    pub fn hsrd_tocnt(&self) -> HSRD_TOCNT_R {
        HSRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSRD_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn hsrd_tocnt(&mut self) -> HSRD_TOCNT_W<DSI_TCCR1_SPEC, 0> {
        HSRD_TOCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR1_SPEC;
impl crate::RegisterSpec for DSI_TCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr1::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr1::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_TCCR1 to value 0"]
impl crate::Resettable for DSI_TCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
