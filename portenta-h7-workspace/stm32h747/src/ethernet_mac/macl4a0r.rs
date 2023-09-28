#[doc = "Register `MACL4A0R` reader"]
pub type R = crate::R<MACL4A0R_SPEC>;
#[doc = "Register `MACL4A0R` writer"]
pub type W = crate::W<MACL4A0R_SPEC>;
#[doc = "Field `L4SP0` reader - L4SP0"]
pub type L4SP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4SP0` writer - L4SP0"]
pub type L4SP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `L4DP0` reader - L4DP0"]
pub type L4DP0_R = crate::FieldReader<u16>;
#[doc = "Field `L4DP0` writer - L4DP0"]
pub type L4DP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4sp0(&mut self) -> L4SP0_W<MACL4A0R_SPEC, 0> {
        L4SP0_W::new(self)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    #[must_use]
    pub fn l4dp0(&mut self) -> L4DP0_W<MACL4A0R_SPEC, 16> {
        L4DP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layer4 address filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl4a0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl4a0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL4A0R_SPEC;
impl crate::RegisterSpec for MACL4A0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl4a0r::R`](R) reader structure"]
impl crate::Readable for MACL4A0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macl4a0r::W`](W) writer structure"]
impl crate::Writable for MACL4A0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACL4A0R to value 0"]
impl crate::Resettable for MACL4A0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
