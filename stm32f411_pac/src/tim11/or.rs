#[doc = "Register `OR` reader"]
pub type R = crate::R<OR_SPEC>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OR_SPEC>;
#[doc = "Field `RMP` reader - Input 1 remapping capability"]
pub type RMP_R = crate::FieldReader;
#[doc = "Field `RMP` writer - Input 1 remapping capability"]
pub type RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Input 1 remapping capability"]
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input 1 remapping capability"]
    #[inline(always)]
    #[must_use]
    pub fn rmp(&mut self) -> RMP_W<OR_SPEC> {
        RMP_W::new(self, 0)
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
#[doc = "option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: u32 = 0;
}
