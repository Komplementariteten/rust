#[doc = "Register `DSI_TCCR3` reader"]
pub type R = crate::R<DSI_TCCR3_SPEC>;
#[doc = "Register `DSI_TCCR3` writer"]
pub type W = crate::W<DSI_TCCR3_SPEC>;
#[doc = "Field `HSWR_TOCNT` reader - HSWR_TOCNT"]
pub type HSWR_TOCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HSWR_TOCNT` writer - HSWR_TOCNT"]
pub type HSWR_TOCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HSWR_TOCNT_R {
        HSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    #[must_use]
    pub fn hswr_tocnt(&mut self) -> HSWR_TOCNT_W<DSI_TCCR3_SPEC, 0> {
        HSWR_TOCNT_W::new(self)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<DSI_TCCR3_SPEC, 24> {
        PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_tccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_tccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_TCCR3_SPEC;
impl crate::RegisterSpec for DSI_TCCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr3::R`](R) reader structure"]
impl crate::Readable for DSI_TCCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr3::W`](W) writer structure"]
impl crate::Writable for DSI_TCCR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_TCCR3 to value 0"]
impl crate::Resettable for DSI_TCCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
