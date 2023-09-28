#[doc = "Register `MDMA_C11CR` reader"]
pub type R = crate::R<MDMA_C11CR_SPEC>;
#[doc = "Register `MDMA_C11CR` writer"]
pub type W = crate::W<MDMA_C11CR_SPEC>;
#[doc = "Field `EN` reader - channel enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - channel enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIE` reader - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_R = crate::BitReader;
#[doc = "Field `CTCIE` writer - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type CTCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRTIE` reader - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
pub type BRTIE_R = crate::BitReader;
#[doc = "Field `BRTIE` writer - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
pub type BRTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTIE` reader - Block Transfer interrupt enable This bit is set and cleared by software."]
pub type BTIE_R = crate::BitReader;
#[doc = "Field `BTIE` writer - Block Transfer interrupt enable This bit is set and cleared by software."]
pub type BTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCIE` reader - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type TCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PL` reader - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
pub type PL_R = crate::FieldReader;
#[doc = "Field `PL` writer - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
pub type PL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `BEX` reader - byte Endianness exchange"]
pub type BEX_R = crate::BitReader;
#[doc = "Field `BEX` writer - byte Endianness exchange"]
pub type BEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HEX` reader - Half word Endianes exchange"]
pub type HEX_R = crate::BitReader;
#[doc = "Field `HEX` writer - Half word Endianes exchange"]
pub type HEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WEX` reader - Word Endianness exchange"]
pub type WEX_R = crate::BitReader;
#[doc = "Field `WEX` writer - Word Endianness exchange"]
pub type WEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRQ` writer - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
pub type SWRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - byte Endianness exchange"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Half word Endianes exchange"]
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Word Endianness exchange"]
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MDMA_C11CR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<MDMA_C11CR_SPEC, 1> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<MDMA_C11CR_SPEC, 2> {
        CTCIE_W::new(self)
    }
    #[doc = "Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn brtie(&mut self) -> BRTIE_W<MDMA_C11CR_SPEC, 3> {
        BRTIE_W::new(self)
    }
    #[doc = "Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn btie(&mut self) -> BTIE_W<MDMA_C11CR_SPEC, 4> {
        BTIE_W::new(self)
    }
    #[doc = "Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<MDMA_C11CR_SPEC, 5> {
        TCIE_W::new(self)
    }
    #[doc = "Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<MDMA_C11CR_SPEC, 6> {
        PL_W::new(self)
    }
    #[doc = "Bit 12 - byte Endianness exchange"]
    #[inline(always)]
    #[must_use]
    pub fn bex(&mut self) -> BEX_W<MDMA_C11CR_SPEC, 12> {
        BEX_W::new(self)
    }
    #[doc = "Bit 13 - Half word Endianes exchange"]
    #[inline(always)]
    #[must_use]
    pub fn hex(&mut self) -> HEX_W<MDMA_C11CR_SPEC, 13> {
        HEX_W::new(self)
    }
    #[doc = "Bit 14 - Word Endianness exchange"]
    #[inline(always)]
    #[must_use]
    pub fn wex(&mut self) -> WEX_W<MDMA_C11CR_SPEC, 14> {
        WEX_W::new(self)
    }
    #[doc = "Bit 16 - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
    #[inline(always)]
    #[must_use]
    pub fn swrq(&mut self) -> SWRQ_W<MDMA_C11CR_SPEC, 16> {
        SWRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c11cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C11CR_SPEC;
impl crate::RegisterSpec for MDMA_C11CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c11cr::R`](R) reader structure"]
impl crate::Readable for MDMA_C11CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c11cr::W`](W) writer structure"]
impl crate::Writable for MDMA_C11CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C11CR to value 0"]
impl crate::Resettable for MDMA_C11CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
