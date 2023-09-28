#[doc = "Register `MACSTSUR` reader"]
pub type R = crate::R<MACSTSUR_SPEC>;
#[doc = "Register `MACSTSUR` writer"]
pub type W = crate::W<MACSTSUR_SPEC>;
#[doc = "Field `TSS` reader - TSS"]
pub type TSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - TSS"]
pub type TSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TSS_W<MACSTSUR_SPEC, 0> {
        TSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System time seconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstsur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstsur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTSUR_SPEC;
impl crate::RegisterSpec for MACSTSUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsur::R`](R) reader structure"]
impl crate::Readable for MACSTSUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macstsur::W`](W) writer structure"]
impl crate::Writable for MACSTSUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSTSUR to value 0"]
impl crate::Resettable for MACSTSUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
