#[doc = "Register `MACMDIODR` reader"]
pub type R = crate::R<MACMDIODR_SPEC>;
#[doc = "Register `MACMDIODR` writer"]
pub type W = crate::W<MACMDIODR_SPEC>;
#[doc = "Field `MD` reader - MD"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MD"]
pub type MD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `RA` reader - RA"]
pub type RA_R = crate::FieldReader<u16>;
#[doc = "Field `RA` writer - RA"]
pub type RA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - MD"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MD"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MACMDIODR_SPEC, 0> {
        MD_W::new(self)
    }
    #[doc = "Bits 16:31 - RA"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MACMDIODR_SPEC, 16> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdiodr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdiodr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMDIODR_SPEC;
impl crate::RegisterSpec for MACMDIODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdiodr::R`](R) reader structure"]
impl crate::Readable for MACMDIODR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmdiodr::W`](W) writer structure"]
impl crate::Writable for MACMDIODR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMDIODR to value 0"]
impl crate::Resettable for MACMDIODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
