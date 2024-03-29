#[doc = "Register `OTG_HS_GRSTCTL` reader"]
pub type R = crate::R<OTG_HS_GRSTCTL_SPEC>;
#[doc = "Register `OTG_HS_GRSTCTL` writer"]
pub type W = crate::W<OTG_HS_GRSTCTL_SPEC>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSRST` reader - HCLK soft reset"]
pub type HSRST_R = crate::BitReader;
#[doc = "Field `HSRST` writer - HCLK soft reset"]
pub type HSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCRST` reader - Host frame counter reset"]
pub type FCRST_R = crate::BitReader;
#[doc = "Field `FCRST` writer - Host frame counter reset"]
pub type FCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFFLSH` reader - RxFIFO flush"]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO flush"]
pub type RXFFLSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFFLSH` reader - TxFIFO flush"]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO flush"]
pub type TXFFLSH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TXFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DMAREQ` reader - DMA request signal enabled for USB OTG HS"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDL` reader - AHB master idle"]
pub type AHBIDL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    pub fn hsrst(&self) -> HSRST_R {
        HSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA request signal enabled for USB OTG HS"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB master idle"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<OTG_HS_GRSTCTL_SPEC, 0> {
        CSRST_W::new(self)
    }
    #[doc = "Bit 1 - HCLK soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsrst(&mut self) -> HSRST_W<OTG_HS_GRSTCTL_SPEC, 1> {
        HSRST_W::new(self)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn fcrst(&mut self) -> FCRST_W<OTG_HS_GRSTCTL_SPEC, 2> {
        FCRST_W::new(self)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<OTG_HS_GRSTCTL_SPEC, 4> {
        RXFFLSH_W::new(self)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<OTG_HS_GRSTCTL_SPEC, 5> {
        TXFFLSH_W::new(self)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<OTG_HS_GRSTCTL_SPEC, 6> {
        TXFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_GRSTCTL_SPEC;
impl crate::RegisterSpec for OTG_HS_GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_grstctl::R`](R) reader structure"]
impl crate::Readable for OTG_HS_GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_grstctl::W`](W) writer structure"]
impl crate::Writable for OTG_HS_GRSTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_GRSTCTL to value 0x2000_0000"]
impl crate::Resettable for OTG_HS_GRSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
