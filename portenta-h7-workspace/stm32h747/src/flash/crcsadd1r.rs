#[doc = "Register `CRCSADD1R` reader"]
pub type R = crate::R<CRCSADD1R_SPEC>;
#[doc = "Register `CRCSADD1R` writer"]
pub type W = crate::W<CRCSADD1R_SPEC>;
#[doc = "Field `CRC_START_ADDR` reader - CRC start address on bank 1"]
pub type CRC_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_START_ADDR` writer - CRC start address on bank 1"]
pub type CRC_START_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W<CRCSADD1R_SPEC, 0> {
        CRC_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crcsadd1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crcsadd1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRCSADD1R_SPEC;
impl crate::RegisterSpec for CRCSADD1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcsadd1r::R`](R) reader structure"]
impl crate::Readable for CRCSADD1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crcsadd1r::W`](W) writer structure"]
impl crate::Writable for CRCSADD1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCSADD1R to value 0"]
impl crate::Resettable for CRCSADD1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
