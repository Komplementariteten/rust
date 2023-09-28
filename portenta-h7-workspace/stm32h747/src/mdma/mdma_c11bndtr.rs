#[doc = "Register `MDMA_C11BNDTR` reader"]
pub type R = crate::R<MDMA_C11BNDTR_SPEC>;
#[doc = "Register `MDMA_C11BNDTR` writer"]
pub type W = crate::W<MDMA_C11BNDTR_SPEC>;
#[doc = "Field `BNDT` reader - block number of data to transfer"]
pub type BNDT_R = crate::FieldReader<u32>;
#[doc = "Field `BNDT` writer - block number of data to transfer"]
pub type BNDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 17, O, u32>;
#[doc = "Field `BRSUM` reader - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRSUM_R = crate::BitReader;
#[doc = "Field `BRSUM` writer - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRSUM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRDUM` reader - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRDUM_R = crate::BitReader;
#[doc = "Field `BRDUM` writer - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BRDUM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRC` reader - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BRC_R = crate::FieldReader<u16>;
#[doc = "Field `BRC` writer - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<MDMA_C11BNDTR_SPEC, 0> {
        BNDT_W::new(self)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brsum(&mut self) -> BRSUM_W<MDMA_C11BNDTR_SPEC, 18> {
        BRSUM_W::new(self)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brdum(&mut self) -> BRDUM_W<MDMA_C11BNDTR_SPEC, 19> {
        BRDUM_W::new(self)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<MDMA_C11BNDTR_SPEC, 20> {
        BRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA Channel x block number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11bndtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11bndtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C11BNDTR_SPEC;
impl crate::RegisterSpec for MDMA_C11BNDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c11bndtr::R`](R) reader structure"]
impl crate::Readable for MDMA_C11BNDTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c11bndtr::W`](W) writer structure"]
impl crate::Writable for MDMA_C11BNDTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C11BNDTR to value 0"]
impl crate::Resettable for MDMA_C11BNDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
