#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<CCMR1_INPUT_SPEC>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<CCMR1_INPUT_SPEC>;
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICPCS` reader - Input capture 1 prescaler"]
pub type ICPCS_R = crate::FieldReader;
#[doc = "Field `ICPCS` writer - Input capture 1 prescaler"]
pub type ICPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type IC1F_R = crate::FieldReader;
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection"]
pub type CC2S_R = crate::FieldReader;
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection"]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2PCS` reader - Input capture 2 prescaler"]
pub type IC2PCS_R = crate::FieldReader;
#[doc = "Field `IC2PCS` writer - Input capture 2 prescaler"]
pub type IC2PCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn icpcs(&self) -> ICPCS_R {
        ICPCS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2pcs(&self) -> IC2PCS_R {
        IC2PCS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<CCMR1_INPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn icpcs(&mut self) -> ICPCS_W<CCMR1_INPUT_SPEC> {
        ICPCS_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<CCMR1_INPUT_SPEC> {
        IC1F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<CCMR1_INPUT_SPEC> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic2pcs(&mut self) -> IC2PCS_W<CCMR1_INPUT_SPEC> {
        IC2PCS_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Input capture 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<CCMR1_INPUT_SPEC> {
        IC2F_W::new(self, 12)
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
#[doc = "capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccmr1_input::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
