#[doc = "Register `MTLOMR` reader"]
pub type R = crate::R<MTLOMR_SPEC>;
#[doc = "Register `MTLOMR` writer"]
pub type W = crate::W<MTLOMR_SPEC>;
#[doc = "Field `DTXSTS` reader - DTXSTS"]
pub type DTXSTS_R = crate::BitReader;
#[doc = "Field `DTXSTS` writer - DTXSTS"]
pub type DTXSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTPRST` reader - CNTPRST"]
pub type CNTPRST_R = crate::BitReader;
#[doc = "Field `CNTPRST` writer - CNTPRST"]
pub type CNTPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTCLR` reader - CNTCLR"]
pub type CNTCLR_R = crate::BitReader;
#[doc = "Field `CNTCLR` writer - CNTCLR"]
pub type CNTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DTXSTS"]
    #[inline(always)]
    #[must_use]
    pub fn dtxsts(&mut self) -> DTXSTS_W<MTLOMR_SPEC, 1> {
        DTXSTS_W::new(self)
    }
    #[doc = "Bit 8 - CNTPRST"]
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<MTLOMR_SPEC, 8> {
        CNTPRST_W::new(self)
    }
    #[doc = "Bit 9 - CNTCLR"]
    #[inline(always)]
    #[must_use]
    pub fn cntclr(&mut self) -> CNTCLR_W<MTLOMR_SPEC, 9> {
        CNTCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLOMR_SPEC;
impl crate::RegisterSpec for MTLOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlomr::R`](R) reader structure"]
impl crate::Readable for MTLOMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtlomr::W`](W) writer structure"]
impl crate::Writable for MTLOMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLOMR to value 0"]
impl crate::Resettable for MTLOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
