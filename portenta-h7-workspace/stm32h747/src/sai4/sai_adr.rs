#[doc = "Register `SAI_ADR` reader"]
pub type R = crate::R<SAI_ADR_SPEC>;
#[doc = "Register `SAI_ADR` writer"]
pub type W = crate::W<SAI_ADR_SPEC>;
#[doc = "Field `DATA` reader - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
pub type DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data A write to this register loads the FIFO provided the FIFO is not full. A read from this register empties the FIFO if the FIFO is not empty."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SAI_ADR_SPEC, 0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_ADR_SPEC;
impl crate::RegisterSpec for SAI_ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_adr::R`](R) reader structure"]
impl crate::Readable for SAI_ADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sai_adr::W`](W) writer structure"]
impl crate::Writable for SAI_ADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_ADR to value 0"]
impl crate::Resettable for SAI_ADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
