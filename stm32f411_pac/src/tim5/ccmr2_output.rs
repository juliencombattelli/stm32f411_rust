#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<CCMR2_OUTPUT_SPEC>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<CCMR2_OUTPUT_SPEC>;
#[doc = "Field `CC3S` reader - CC3S"]
pub type CC3S_R = crate::FieldReader;
#[doc = "Field `CC3S` writer - CC3S"]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC3FE` reader - OC3FE"]
pub type OC3FE_R = crate::BitReader;
#[doc = "Field `OC3FE` writer - OC3FE"]
pub type OC3FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - OC3PE"]
pub type OC3PE_R = crate::BitReader;
#[doc = "Field `OC3PE` writer - OC3PE"]
pub type OC3PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M` reader - OC3M"]
pub type OC3M_R = crate::FieldReader;
#[doc = "Field `OC3M` writer - OC3M"]
pub type OC3M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - OC3CE"]
pub type OC3CE_R = crate::BitReader;
#[doc = "Field `OC3CE` writer - OC3CE"]
pub type OC3CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4S` reader - CC4S"]
pub type CC4S_R = crate::FieldReader;
#[doc = "Field `CC4S` writer - CC4S"]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC4FE` reader - OC4FE"]
pub type OC4FE_R = crate::BitReader;
#[doc = "Field `OC4FE` writer - OC4FE"]
pub type OC4FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - OC4PE"]
pub type OC4PE_R = crate::BitReader;
#[doc = "Field `OC4PE` writer - OC4PE"]
pub type OC4PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - OC4M"]
pub type OC4M_R = crate::FieldReader;
#[doc = "Field `OC4M` writer - OC4M"]
pub type OC4M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - OC4CE"]
pub type OC4CE_R = crate::BitReader;
#[doc = "Field `OC4CE` writer - OC4CE"]
pub type OC4CE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC4CE"]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_OUTPUT_SPEC> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<CCMR2_OUTPUT_SPEC> {
        OC3FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<CCMR2_OUTPUT_SPEC> {
        OC3PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OC3M_W<CCMR2_OUTPUT_SPEC> {
        OC3M_W::new(self, 4)
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<CCMR2_OUTPUT_SPEC> {
        OC3CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_OUTPUT_SPEC> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<CCMR2_OUTPUT_SPEC> {
        OC4FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<CCMR2_OUTPUT_SPEC> {
        OC4PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OC4M_W<CCMR2_OUTPUT_SPEC> {
        OC4M_W::new(self, 12)
    }
    #[doc = "Bit 15 - OC4CE"]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<CCMR2_OUTPUT_SPEC> {
        OC4CE_W::new(self, 15)
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
#[doc = "capture/compare mode register 2 (output mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr2_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for CCMR2_OUTPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
