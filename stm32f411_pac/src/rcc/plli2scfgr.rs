#[doc = "Register `PLLI2SCFGR` reader"]
pub type R = crate::R<PLLI2SCFGR_SPEC>;
#[doc = "Register `PLLI2SCFGR` writer"]
pub type W = crate::W<PLLI2SCFGR_SPEC>;
#[doc = "Field `PLLI2SNx` reader - PLLI2S multiplication factor for VCO"]
pub type PLLI2SNX_R = crate::FieldReader<u16>;
#[doc = "Field `PLLI2SNx` writer - PLLI2S multiplication factor for VCO"]
pub type PLLI2SNX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PLLI2SRx` reader - PLLI2S division factor for I2S clocks"]
pub type PLLI2SRX_R = crate::FieldReader;
#[doc = "Field `PLLI2SRx` writer - PLLI2S division factor for I2S clocks"]
pub type PLLI2SRX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2snx(&self) -> PLLI2SNX_R {
        PLLI2SNX_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2srx(&self) -> PLLI2SRX_R {
        PLLI2SRX_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plli2snx(&mut self) -> PLLI2SNX_W<PLLI2SCFGR_SPEC> {
        PLLI2SNX_W::new(self, 6)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srx(&mut self) -> PLLI2SRX_W<PLLI2SCFGR_SPEC> {
        PLLI2SRX_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PLLI2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plli2scfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plli2scfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLI2SCFGR_SPEC;
impl crate::RegisterSpec for PLLI2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plli2scfgr::R`](R) reader structure"]
impl crate::Readable for PLLI2SCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plli2scfgr::W`](W) writer structure"]
impl crate::Writable for PLLI2SCFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGR_SPEC {
    const RESET_VALUE: u32 = 0x2000_3000;
}
