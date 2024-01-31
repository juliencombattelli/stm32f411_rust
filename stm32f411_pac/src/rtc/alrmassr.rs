#[doc = "Register `ALRMASSR` reader"]
pub type R = crate::R<ALRMASSR_SPEC>;
#[doc = "Register `ALRMASSR` writer"]
pub type W = crate::W<ALRMASSR_SPEC>;
#[doc = "Field `SS` reader - Sub seconds value"]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value"]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit"]
pub type MASKSS_R = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit"]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRMASSR_SPEC> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit"]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMASSR_SPEC> {
        MASKSS_W::new(self, 24)
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
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmassr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmassr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMASSR_SPEC;
impl crate::RegisterSpec for ALRMASSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmassr::R`](R) reader structure"]
impl crate::Readable for ALRMASSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrmassr::W`](W) writer structure"]
impl crate::Writable for ALRMASSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRMASSR to value 0"]
impl crate::Resettable for ALRMASSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
