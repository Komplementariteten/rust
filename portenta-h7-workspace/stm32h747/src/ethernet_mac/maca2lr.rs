#[doc = "Register `MACA2LR` reader"]
pub type R = crate::R<MACA2LR_SPEC>;
#[doc = "Register `MACA2LR` writer"]
pub type W = crate::W<MACA2LR_SPEC>;
#[doc = "Field `ADDRLO` reader - ADDRLO"]
pub type ADDRLO_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO` writer - ADDRLO"]
pub type ADDRLO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDRLO"]
    #[inline(always)]
    #[must_use]
    pub fn addrlo(&mut self) -> ADDRLO_W<MACA2LR_SPEC, 0> {
        ADDRLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2LR_SPEC;
impl crate::RegisterSpec for MACA2LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2lr::R`](R) reader structure"]
impl crate::Readable for MACA2LR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure"]
impl crate::Writable for MACA2LR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA2LR to value 0xffff_ffff"]
impl crate::Resettable for MACA2LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
