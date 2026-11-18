import InputOtp from "./InputOtp.astro";
import InputOtpGroup from "./InputOtpGroup.astro";
import InputOtpSeparator from "./InputOtpSeparator.astro";
import InputOtpSlot from "./InputOtpSlot.astro";
import type { InputOtpChangeEvent } from "./InputOtpTypes";

const REGEXP_ONLY_DIGITS_AND_CHARS = /^[A-Za-z0-9]+$/;
const REGEXP_ONLY_DIGITS = /^[0-9]+$/;

export {
  InputOtp,
  type InputOtpChangeEvent,
  InputOtpGroup,
  InputOtpSeparator,
  InputOtpSlot,
  REGEXP_ONLY_DIGITS,
  REGEXP_ONLY_DIGITS_AND_CHARS,
};

export default {
  Root: InputOtp,
  Group: InputOtpGroup,
  Separator: InputOtpSeparator,
  Slot: InputOtpSlot,
};
