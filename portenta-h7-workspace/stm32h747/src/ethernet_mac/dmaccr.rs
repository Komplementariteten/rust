#[doc = "Register `DMACCR` reader"]
pub type R = crate::R<DMACCR_SPEC>;
#[doc = "Register `DMACCR` writer"]
pub type W = crate::W<DMACCR_SPEC>;
#[doc = "Field `MSS` reader - Maximum Segment Size"]
pub type MSS_R = crate::FieldReader<u16>;
#[doc = "Field `MSS` writer - Maximum Segment Size"]
pub type MSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `PBLX8` reader - 8xPBL mode"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLX8` writer - 8xPBL mode"]
pub type PBLX8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<DMACCR_SPEC, 0> {
        MSS_W::new(self)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<DMACCR_SPEC, 16> {
        PBLX8_W::new(self)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMACCR_SPEC, 18> {
        DSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCR_SPEC;
impl crate::RegisterSpec for DMACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccr::R`](R) reader structure"]
impl crate::Readable for DMACCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaccr::W`](W) writer structure"]
impl crate::Writable for DMACCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACCR to value 0"]
impl crate::Resettable for DMACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
