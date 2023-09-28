#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Field `BRR_0_3` reader - DIV_Fraction"]
pub type BRR_0_3_R = crate::FieldReader;
#[doc = "Field `BRR_0_3` writer - DIV_Fraction"]
pub type BRR_0_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `BRR_4_15` reader - DIV_Mantissa"]
pub type BRR_4_15_R = crate::FieldReader<u16>;
#[doc = "Field `BRR_4_15` writer - DIV_Mantissa"]
pub type BRR_4_15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn brr_0_3(&self) -> BRR_0_3_R {
        BRR_0_3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn brr_4_15(&self) -> BRR_4_15_R {
        BRR_4_15_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    #[must_use]
    pub fn brr_0_3(&mut self) -> BRR_0_3_W<BRR_SPEC, 0> {
        BRR_0_3_W::new(self)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    #[must_use]
    pub fn brr_4_15(&mut self) -> BRR_4_15_W<BRR_SPEC, 4> {
        BRR_4_15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
