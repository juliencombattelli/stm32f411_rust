#[doc = "Register `CALIBR` reader"]
pub type R = crate::R<CALIBR_SPEC>;
#[doc = "Register `CALIBR` writer"]
pub type W = crate::W<CALIBR_SPEC>;
#[doc = "Field `DC` reader - Digital calibration"]
pub type DC_R = crate::FieldReader;
#[doc = "Field `DC` writer - Digital calibration"]
pub type DC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCS` reader - Digital calibration sign"]
pub type DCS_R = crate::BitReader;
#[doc = "Field `DCS` writer - Digital calibration sign"]
pub type DCS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Digital calibration"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Digital calibration sign"]
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Digital calibration"]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<CALIBR_SPEC> {
        DC_W::new(self, 0)
    }
    #[doc = "Bit 7 - Digital calibration sign"]
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DCS_W<CALIBR_SPEC> {
        DCS_W::new(self, 7)
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
#[doc = "calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calibr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calibr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALIBR_SPEC;
impl crate::RegisterSpec for CALIBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calibr::R`](R) reader structure"]
impl crate::Readable for CALIBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calibr::W`](W) writer structure"]
impl crate::Writable for CALIBR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALIBR to value 0"]
impl crate::Resettable for CALIBR_SPEC {
    const RESET_VALUE: u32 = 0;
}
