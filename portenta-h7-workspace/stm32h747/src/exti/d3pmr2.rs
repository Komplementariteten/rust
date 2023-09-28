#[doc = "Register `D3PMR2` reader"]
pub type R = crate::R<D3PMR2_SPEC>;
#[doc = "Register `D3PMR2` writer"]
pub type W = crate::W<D3PMR2_SPEC>;
#[doc = "Field `MR34` reader - D3 Pending Mask on Event input x+32"]
pub type MR34_R = crate::BitReader;
#[doc = "Field `MR34` writer - D3 Pending Mask on Event input x+32"]
pub type MR34_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR35` reader - D3 Pending Mask on Event input x+32"]
pub type MR35_R = crate::BitReader;
#[doc = "Field `MR35` writer - D3 Pending Mask on Event input x+32"]
pub type MR35_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR41` reader - D3 Pending Mask on Event input x+32"]
pub type MR41_R = crate::BitReader;
#[doc = "Field `MR41` writer - D3 Pending Mask on Event input x+32"]
pub type MR41_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR48` reader - D3 Pending Mask on Event input x+32"]
pub type MR48_R = crate::BitReader;
#[doc = "Field `MR48` writer - D3 Pending Mask on Event input x+32"]
pub type MR48_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR49` reader - D3 Pending Mask on Event input x+32"]
pub type MR49_R = crate::BitReader;
#[doc = "Field `MR49` writer - D3 Pending Mask on Event input x+32"]
pub type MR49_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR50` reader - D3 Pending Mask on Event input x+32"]
pub type MR50_R = crate::BitReader;
#[doc = "Field `MR50` writer - D3 Pending Mask on Event input x+32"]
pub type MR50_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR51` reader - D3 Pending Mask on Event input x+32"]
pub type MR51_R = crate::BitReader;
#[doc = "Field `MR51` writer - D3 Pending Mask on Event input x+32"]
pub type MR51_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR52` reader - D3 Pending Mask on Event input x+32"]
pub type MR52_R = crate::BitReader;
#[doc = "Field `MR52` writer - D3 Pending Mask on Event input x+32"]
pub type MR52_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MR53` reader - D3 Pending Mask on Event input x+32"]
pub type MR53_R = crate::BitReader;
#[doc = "Field `MR53` writer - D3 Pending Mask on Event input x+32"]
pub type MR53_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<D3PMR2_SPEC, 2> {
        MR34_W::new(self)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<D3PMR2_SPEC, 3> {
        MR35_W::new(self)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr41(&mut self) -> MR41_W<D3PMR2_SPEC, 9> {
        MR41_W::new(self)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr48(&mut self) -> MR48_W<D3PMR2_SPEC, 16> {
        MR48_W::new(self)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr49(&mut self) -> MR49_W<D3PMR2_SPEC, 17> {
        MR49_W::new(self)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr50(&mut self) -> MR50_W<D3PMR2_SPEC, 18> {
        MR50_W::new(self)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr51(&mut self) -> MR51_W<D3PMR2_SPEC, 19> {
        MR51_W::new(self)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr52(&mut self) -> MR52_W<D3PMR2_SPEC, 20> {
        MR52_W::new(self)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    #[must_use]
    pub fn mr53(&mut self) -> MR53_W<D3PMR2_SPEC, 21> {
        MR53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3pmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3pmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3PMR2_SPEC;
impl crate::RegisterSpec for D3PMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr2::R`](R) reader structure"]
impl crate::Readable for D3PMR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3pmr2::W`](W) writer structure"]
impl crate::Writable for D3PMR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3PMR2 to value 0"]
impl crate::Resettable for D3PMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
